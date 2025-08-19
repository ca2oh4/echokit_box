#[allow(unused)]
fn print_stack_high() {
    let stack_high =
        unsafe { esp_idf_svc::sys::uxTaskGetStackHighWaterMark2(std::ptr::null_mut()) };
    log::info!("Stack high: {}", stack_high);
}

use crate::{app::Event, protocol::ServerEvent};
use std::sync::mpsc;
use tokio_websockets::Message;

use esp_idf_svc::{
    io::EspIOError,
    ws::client::{
        EspWebSocketClient, EspWebSocketClientConfig, FrameType, WebSocketEvent, WebSocketEventType,
    },
};

pub struct Server<'a> {
    pub uri: String,
    timeout: std::time::Duration,
    client: EspWebSocketClient<'a>,
    receiver: mpsc::Receiver<ServerEvent>,
}

#[derive(Debug, PartialEq)]
enum ExampleEvent {
    Connected,
    MessageReceived,
    Closed,
}

impl Server<'_> {
    pub async fn new(uri: String) -> anyhow::Result<Self> {
        let mut conf = EspWebSocketClientConfig::default();
        conf.crt_bundle_attach = Some(esp_idf_svc::sys::esp_crt_bundle_attach);
        conf.use_global_ca_store = true;
        let (tx, rx) = mpsc::channel::<ExampleEvent>();
        let (tx2, rx2) = mpsc::channel::<ServerEvent>();

        let timeout = std::time::Duration::from_secs(10);
        let client = EspWebSocketClient::new(uri.as_ref(), &conf, timeout, move |event| {
            Self::ws_handle_event(&tx, &tx2, event)
        })?;
        log::info!("ws recv: {:?}", rx.recv());
        log::info!("ws is_connected: {:?}", client.is_connected());

        Ok(Self {
            uri,
            timeout,
            client,
            receiver: rx2,
        })
    }

    pub fn set_timeout(&mut self, timeout: std::time::Duration) {
        self.timeout = timeout;
    }

    pub fn send(&mut self, msg: Message) -> anyhow::Result<()> {
        if msg.is_binary() {
            self.client
                .send(FrameType::Binary(false), msg.as_payload().iter().as_slice())
        } else if msg.is_text() {
            self.client
                .send(FrameType::Text(false), msg.as_text().unwrap().as_ref())
        } else {
            Ok(log::warn!("unknow type msg"))
        }
        .expect("TODO: panic message");

        Ok(())
    }

    pub async fn recv(&mut self) -> anyhow::Result<Event> {
        let evt = self.receiver.recv()?;
        Ok(Event::ServerEvent(evt))
    }

    fn ws_handle_event(
        tx: &mpsc::Sender<ExampleEvent>,
        tx2: &mpsc::Sender<ServerEvent>,
        event: &Result<WebSocketEvent, EspIOError>,
    ) {
        if let Ok(event) = event {
            match event.event_type {
                WebSocketEventType::BeforeConnect => {
                    log::info!("Websocket before connect");
                }
                WebSocketEventType::Connected => {
                    log::info!("Websocket connected");
                    tx.send(ExampleEvent::Connected).ok();
                }
                WebSocketEventType::Disconnected => {
                    log::info!("Websocket disconnected");
                }
                WebSocketEventType::Close(reason) => {
                    log::info!("Websocket close, reason: {reason:?}");
                }
                WebSocketEventType::Closed => {
                    log::info!("Websocket closed");
                    tx.send(ExampleEvent::Closed).ok();
                }
                WebSocketEventType::Text(text) => {
                    log::info!("Websocket recv, text: {text}");
                    if text == "Hello, World!" {
                        tx.send(ExampleEvent::MessageReceived).ok();
                    }
                }
                WebSocketEventType::Binary(binary) => {
                    log::info!("Websocket recv, binary: {:?}", binary.len());
                    let evt = rmp_serde::from_slice::<ServerEvent>(&binary)
                        .map_err(|e| anyhow::anyhow!("Failed to deserialize binary data: {}", e))
                        .unwrap();
                    tx2.send(evt).unwrap();
                }
                WebSocketEventType::Ping => {
                    log::info!("Websocket ping");
                }
                WebSocketEventType::Pong => {
                    log::info!("Websocket pong");
                }
            }
        }
    }
}
