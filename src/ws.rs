#[allow(unused)]
fn print_stack_high() {
    let stack_high =
        unsafe { esp_idf_svc::sys::uxTaskGetStackHighWaterMark2(std::ptr::null_mut()) };
    log::info!("Stack high: {}", stack_high);
}

use std::fmt::Debug;
use crate::{app::Event, protocol::ServerEvent};
use esp_idf_svc::sntp;
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_websockets::{Connector, Error, Message};

pub struct Server {
    timeout: std::time::Duration,
    ws: tokio_websockets::WebSocketStream<tokio_websockets::MaybeTlsStream<tokio::net::TcpStream>>,
}

impl Server {
    pub async fn new(uri: String) -> anyhow::Result<Self> {
        log::info!("ws connecting to {}", uri);
        sync_time();

        log::info!("ws connecting step0");
        let (scheme, rest) = uri.split_once("://").unwrap();
        let default_port = match scheme {
            "wss" => 443,
            "ws" => 80,
            _ => 80,
        };

        // 提取 host[:port] 部分
        let host_port = rest.split('/').next().unwrap();
        let (host, port) = if let Some((h, p)) = host_port.split_once(':') {
            (h, p.parse::<u16>().unwrap_or(default_port))
        } else {
            (host_port, default_port)
        };
        log::info!("ws connecting to {}:{}", host, port);
        log::info!("ws connecting step3");
        let tcp_stream = TcpStream::connect(format!("{host}:{port}")).await?;
        log::info!("ws connecting step4");
        let provider = Arc::new(rustls_rustcrypto::provider());
        log::info!("ws connecting step5");
        let connector = Connector::new_rustls_with_crypto_provider(provider)?;
        log::info!("ws connecting step6");
        let tls_stream = connector.wrap(host, tcp_stream).await?;
        log::info!("ws connecting step7");

        let (ws, resp) = tokio_websockets::ClientBuilder::new()
            .uri(&uri)?
            .connect_on(tls_stream)
            .await?;

        log::info!(
            "ws resp status: {:?}, headers: {:?} ",
            resp.status(),
            resp.headers()
        );

        let timeout = std::time::Duration::from_secs(30);

        Ok(Self { timeout, ws })
    }

    pub fn set_timeout(&mut self, timeout: std::time::Duration) {
        self.timeout = timeout;
    }

    pub async fn send(&mut self, msg: Message) -> anyhow::Result<()> {
        tokio::time::timeout(self.timeout, self.ws.send(msg))
            .map_err(|_| anyhow::anyhow!("Timeout sending message"))
            .await??;
        Ok(())
    }

    pub async fn recv(&mut self) -> anyhow::Result<Event> {
        let msg = self
            .ws
            .next()
            .await
            .ok_or_else(|| anyhow::anyhow!("WS channel closed"))??;

        if msg.is_binary() {
            let payload = msg.into_payload();
            let evt = rmp_serde::from_slice::<ServerEvent>(&payload)
                .map_err(|e| anyhow::anyhow!("Failed to deserialize binary data: {}", e))?;
            Ok(Event::ServerEvent(evt))
        } else {
            Err(anyhow::anyhow!("Invalid message type"))
        }
    }
}

fn sync_time() {
    log::info!("Sync time");
    show_now();
    let lsntp = sntp::EspSntp::new_default().unwrap();
    loop {
        let status = lsntp.get_sync_status();
        log::info!("sntp sync status {:?}", status);
        if status == sntp::SyncStatus::Completed{
            show_now();
            break;
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    log::info!("NTP time synchronized!");
}

fn show_now(){
    let now = std::time::SystemTime::now();
    log::info!("now time: {:?}", now);
}