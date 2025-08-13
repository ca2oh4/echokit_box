use esp_idf_svc::hal::adc::{Adc, ADC1};
use esp_idf_svc::hal::gpio::{AnyIOPin, PinDriver};
// use esp_idf_svc::hal::gpio::a;

pub fn run(adc1: ADC1, adc_pin: AnyIOPin, led_pin: AnyIOPin) {
    let mut adc = Adc1::new(adc1);
    let mut adc_channel = adc.enable(&adc_pin)?;

    // 配置 LED，GPIO13 输出
    let mut led = PinDriver::output(led_pin).unwrap();

    // 声音阈值，ADC 读数范围 0~4095
    const SOUND_THRESHOLD: u16 = 1000;

    println!("Start sound detection...");

    loop {
        // 读取 ADC 电压值
        let sample: u16 = adc.read(&mut adc_channel)?;
        println!("ADC sample: {}", sample);

        if sample > SOUND_THRESHOLD {
            led.set_high().unwrap();
            println!("Sound detected: LED ON");
        } else {
            led.set_low().unwrap();
            println!("No sound: LED OFF");
        }

        // 延迟 100ms
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
