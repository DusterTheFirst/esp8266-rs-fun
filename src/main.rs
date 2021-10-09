#![no_std]
#![no_main]

use esp8266_hal::{gpio::*, prelude::*, target::Peripherals};
use panic_halt as _;
use xtensa_lx::mutex::{CriticalSectionMutex, Mutex};

#[entry]
fn start() -> ! {
    main();
}

static BUZZER: CriticalSectionMutex<Option<Gpio4<Output<PushPull>>>> =
    CriticalSectionMutex::new(None);
static RED_LED: CriticalSectionMutex<Option<Gpio5<Output<PushPull>>>> =
    CriticalSectionMutex::new(None);

fn main() -> ! {
    let dp = Peripherals::take().unwrap();

    let pins = dp.GPIO.split();

    let mut builtin_led = pins.gpio2.into_push_pull_output();
    let mut red_led = pins.gpio5.into_push_pull_output();
    let mut buzzer = pins.gpio4.into_push_pull_output();

    let mut button1 = pins.gpio14.into_pull_up_input();
    let mut button2 = pins.gpio12.into_pull_up_input();

    button1.set_interrupt_mode(InterruptMode::BothEdges);

    let (mut timer1, _) = dp.TIMER.timers();

    builtin_led.set_high().unwrap();
    red_led.set_high().unwrap();
    buzzer.set_low().unwrap();

    (&BUZZER).lock(|buzzer_lock| *buzzer_lock = Some(buzzer));
    (&RED_LED).lock(|led_lock| *led_lock = Some(red_led));

    loop {
        timer1.delay_ms(100);
        builtin_led.toggle().unwrap();
        // red_led.toggle().unwrap();
    }
}

#[interrupt(GPIO)]
fn button_interrupt() {
    (&BUZZER).lock(|buzzer_lock| {
        let buzzer = buzzer_lock.as_mut().unwrap();

        buzzer.toggle().unwrap();
    });

    (&RED_LED).lock(|led_lock| {
        let red_led = led_lock.as_mut().unwrap();

        red_led.toggle().unwrap();
    });
}
