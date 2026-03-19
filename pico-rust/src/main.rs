#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use hal::pac;
use panic_halt as _;
use rp235x_hal as hal;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let sio = hal::Sio::new(pac.SIO);

    let _clocks = hal::clocks::init_clocks_and_plls(
        12_000_000,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led = pins.gpio25.into_push_pull_output();

    loop {
        led.set_high().unwrap();
        cortex_m::asm::delay(10_000_000);

        led.set_low().unwrap();
        cortex_m::asm::delay(10_000_000);
    }
}
