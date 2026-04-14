//! DHT11 Sensor Driver for Raspberry Pi Pico 2W with Embassy
//!
//! Connection:
//! - DHT11 GND -> Pico GND
//! - DHT11 VCC -> Pico 3.3V
//! - DHT11 DATA -> Pico GP2 (configurable)
//!
//! The DHT11 uses a single-wire protocol on the data pin.

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::Flex;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler as UsbInterruptHandler};
use embassy_time::{Duration, Instant, Timer};
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::{Builder, UsbDevice};
use heapless::String;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

/// DHT11 sensor reading
#[derive(Debug, Clone, Copy)]
pub struct DhtReading {
    pub humidity: f32,
    pub temperature: f32,
}

#[derive(Debug, Format)]
pub enum DhtError {
    NoResponse,
    ReadTimeout,
    ChecksumMismatch,
}

/// DHT11 driver
pub struct Dht11<'d> {
    pin: Flex<'d>,
}

impl<'d> Dht11<'d> {
    pub fn new(pin: Flex<'d>) -> Self {
        Self { pin }
    }

    /// Read temperature and humidity from DHT11
    pub async fn read(&mut self) -> Result<DhtReading, DhtError> {
        // Configure pin as output
        self.pin.set_as_output();

        // Send start signal: pull low for 18ms
        self.pin.set_low();
        Timer::after(Duration::from_millis(18)).await;

        // Release pin (let it float high via pullup)
        self.pin.set_as_input();

        // Wait for DHT11 to respond (should go low)
        if !self.wait_for_level(false, 100).await {
            return Err(DhtError::NoResponse);
        }

        // Wait for DHT11 to release (should go high)
        if !self.wait_for_level(true, 100).await {
            return Err(DhtError::NoResponse);
        }

        let mut data = [0u8; 5];
        for byte_index in 0..5 {
            let mut byte = 0u8;

            for _bit_index in 0..8 {
                if !self.wait_for_level(false, 100).await {
                    return Err(DhtError::ReadTimeout);
                }

                if !self.wait_for_level(true, 100).await {
                    return Err(DhtError::ReadTimeout);
                }

                let start = Instant::now();
                while self.pin.is_high() && start.elapsed() < Duration::from_micros(100) {
                    // Tight loop to measure pulse width
                }

                let duration = start.elapsed();

                byte <<= 1;
                if duration > Duration::from_micros(40) {
                    byte |= 1;
                }
            }

            data[byte_index] = byte;
        }

        let checksum = data[0]
            .wrapping_add(data[1])
            .wrapping_add(data[2])
            .wrapping_add(data[3]);

        if checksum != data[4] {
            return Err(DhtError::ChecksumMismatch);
        }

        let humidity = data[0] as f32 + (data[1] as f32) / 100.0;

        let temp_integer = data[2] as f32;
        let temp_decimal = (data[3] as f32) / 100.0;
        let mut temperature = temp_integer + temp_decimal;

        if data[2] & 0x80 != 0 {
            temperature = -temperature;
        }

        Ok(DhtReading {
            humidity,
            temperature,
        })
    }

    /// Wait for pin to reach a specific level within timeout
    pub async fn wait_for_level(&mut self, level: bool, timeout_us: u32) -> bool {
        let start = Instant::now();
        let timeout = Duration::from_micros(timeout_us as u64);

        loop {
            if start.elapsed() > timeout {
                return false;
            }

            let pin_level = self.pin.is_high();
            if (level && pin_level) || (!level && !pin_level) {
                return true;
            }

            Timer::after(Duration::from_micros(1)).await;
        }
    }
}

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => UsbInterruptHandler<USB>;
});

#[embassy_executor::task]
async fn usb_task(mut usb: UsbDevice<'static, Driver<'static, USB>>) {
    usb.run().await;
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // USB Setup
    static CONFIG_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static BOS_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();
    static STATE: StaticCell<State> = StaticCell::new();

    let driver = Driver::new(p.USB, Irqs);

    let mut usb_builder = Builder::new(
        driver,
        embassy_usb::Config::new(0xc0de, 0xcafe),
        CONFIG_DESC.init([0; 256]),
        BOS_DESC.init([0; 256]),
        &mut [],
        CONTROL_BUF.init([0; 64]),
    );

    let mut class = CdcAcmClass::new(&mut usb_builder, STATE.init(State::new()), 64);

    let usb = usb_builder.build();
    spawner.spawn(unwrap!(usb_task(usb)));

    let dht_pin = Flex::new(p.PIN_2);
    let mut dht11 = Dht11::new(dht_pin);

    Timer::after_secs(2).await; // Give USB time to enumerate

    loop {
        // Simple test message
        let test_msg = b"Hello from Pico 2W! DHT11 is connected.\r\n";
        let _ = class.write_packet(test_msg).await;

        // Try to read DHT11
        match dht11.read().await {
            Ok(reading) => {
                info!(
                    "Temp: {}°C, Hum: {}%",
                    reading.temperature, reading.humidity
                );
                let msg = format_args!(
                    "Temperature: {:.1}°C, Humidity: {:.1}%\r\n",
                    reading.temperature, reading.humidity
                );
                let mut text: String<64> = String::new();
                let _ = core::fmt::write(&mut text, msg);
                let _ = class.write_packet(text.as_bytes()).await;
            }
            Err(e) => {
                info!("DHT11 error: {:?}", e);
                let _ = class.write_packet(b"DHT11 read error!\r\n").await;
            }
        }

        Timer::after_secs(2).await;
    }
}
