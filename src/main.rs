#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;

use nrf52840_hal::gpio::{p0, Level};
use nrf52840_hal::prelude::{InputPin, OutputPin, _embedded_hal_blocking_delay_DelayMs};

#[entry]
fn main() -> ! {
    let peripherals = nrf52840_hal::pac::Peripherals::take().unwrap();
    let pins = p0::Parts::new(peripherals.P0);

    // the LED will turn on when the pin output level is low
    let mut led = pins.p0_13.degrade().into_push_pull_output(Level::Low);

    // pin 11 corresponds to button 1 on the nRF52840DK
    let button1 = pins.p0_11.into_pullup_input();

    let sense_when_goes_to = Level::Low;
    let p0_register_block = nrf52840_hal::pac::P0::ptr();
    unsafe { &(*p0_register_block).pin_cnf[12] }.write(|w| {
        w.dir().input();
        w.input().connect();
        w.pull().pullup();
        w.drive().s0s1();

        match sense_when_goes_to {
            Level::Low => w.sense().low(),
            Level::High => w.sense().high(),
        };

        w
    });

    // the solution that forks the hall would instead use
    // pins.p0_12.into_pullup_sense_input(Level::Low);

    let core = nrf52840_hal::pac::CorePeripherals::take().unwrap();
    let mut delay = nrf52840_hal::Delay::new(core.SYST);

    loop {
        // when button 1 is pressed
        if button1.is_low() == Ok(true) {
            // turn led off
            led.set_high().unwrap();

            // optionally, wait till all work is "done" before proceding
            // delay.delay_ms(100u16);

            peripherals
                .POWER
                .systemoff
                .write(|w| w.systemoff().set_bit());

            // systemoff takes some time, so we cannot break or return
            // That would cause undefined behavior
            // so we just loop for a while, until the system turns off
            loop {
                delay.delay_ms(1000u16);
            }
        }
    }
}
