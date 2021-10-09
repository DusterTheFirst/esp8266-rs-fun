#![no_std]
#![no_main]

use core::fmt::Write;
use esp8266_hal::{
    gpio::*,
    interrupt::{enable_interrupt, InterruptType},
    prelude::*,
    target::Peripherals,
    uart::UART0Serial,
};
use panic_halt as _;
use xtensa_lx::mutex::{CriticalSectionMutex, Mutex};

use crate::time::{current_millis, initialize_timekeeping};

mod time;

#[entry]
fn start() -> ! {
    main();
}

#[interrupt]
fn gpio() {
    gpio_debounce();
}

struct GpioInterruptData {
    buzzer: Gpio4<Output<PushPull>>,
    red_led: Gpio5<Output<PushPull>>,
    button1: Gpio14<Input<PullUp>>,
    button2: Gpio12<Input<PullUp>>,
}

static INTERRUPT_DATA: CriticalSectionMutex<Option<GpioInterruptData>> =
    CriticalSectionMutex::new(None);

static SERIAL: CriticalSectionMutex<Option<UART0Serial>> = CriticalSectionMutex::new(None);

fn main() -> ! {
    let dp = Peripherals::take().unwrap();

    let pins = dp.GPIO.split();

    let mut serial = dp
        .UART0
        .serial(pins.gpio1.into_uart(), pins.gpio3.into_uart());

    write!(serial, "\r\nInitialized:\r\n").unwrap();

    let mut builtin_led = pins.gpio2.into_push_pull_output();
    let mut red_led = pins.gpio5.into_push_pull_output();
    let mut buzzer = pins.gpio4.into_push_pull_output();

    let mut button1 = pins.gpio14.into_pull_up_input();
    let mut button2 = pins.gpio12.into_pull_up_input();

    button1.set_interrupt_mode(InterruptMode::NegativeEdge);
    button2.set_interrupt_mode(InterruptMode::NegativeEdge);
    enable_interrupt(InterruptType::GPIO);

    let (timer1, mut timer2) = dp.TIMER.timers();

    initialize_timekeeping(timer1);

    builtin_led.set_high().unwrap();
    red_led.set_high().unwrap();
    buzzer.set_low().unwrap();

    (&INTERRUPT_DATA).lock(|lock| {
        *lock = Some(GpioInterruptData {
            buzzer,
            red_led,
            button1,
            button2,
        })
    });

    (&SERIAL).lock(|serial_lock| *serial_lock = Some(serial));

    loop {
        timer2.delay_ms(500);
        builtin_led.toggle().unwrap();

        (&SERIAL)
            .lock(|serial_lock| {
                writeln!(
                    serial_lock.as_mut().unwrap(),
                    "Current ms: {}",
                    current_millis()
                )
            })
            .unwrap();
        // red_led.toggle().unwrap();
    }
}

fn gpio_debounce() {
    static NEXT_VALID: CriticalSectionMutex<u128> = CriticalSectionMutex::new(0);
    const INTERVAL: u128 = 500;

    let millis = current_millis();

    if millis > (&NEXT_VALID).lock(|f| *f) {
        (&NEXT_VALID).lock(|f| *f = millis + INTERVAL);

        button_interrupt();
    }
}

fn button_interrupt() {
    (&SERIAL)
        .lock(|serial| write!(serial.as_mut().unwrap(), "INTERRUPT"))
        .unwrap();

    (&INTERRUPT_DATA).lock(|lock| {
        let GpioInterruptData {
            buzzer,
            red_led,
            button1,
            button2,
        } = lock.as_mut().unwrap();

        button1.clear_interrupt();
        button2.clear_interrupt();

        red_led.toggle().unwrap();
    });
}
