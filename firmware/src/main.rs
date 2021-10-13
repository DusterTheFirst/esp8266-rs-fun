#![no_std]
#![no_main]

use core::{
    hint, panic,
    sync::atomic::{self, Ordering},
};
use defmt::{error, info, trace};
use esp8266_hal::{gpio::*, prelude::*, target::Peripherals};
use micromath::F32Ext;

use crate::{
    logger::{init_logger, PanicInfo},
    music::{JERK_IT_OUT, THE_GOOD_LIFE},
    time::initialize_timekeeping,
};

mod logger;
mod music;
mod note;
mod time;

#[panic_handler]
fn panic_handler(info: &panic::PanicInfo) -> ! {
    error!("PANIC: {}", PanicInfo::from(info));

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

#[entry]
fn start() -> ! {
    main();
}

fn main() -> ! {
    let dp = Peripherals::take().unwrap();

    let pins = dp.GPIO.split();

    let mut serial = dp
        .UART0
        .serial(pins.gpio1.into_uart(), pins.gpio3.into_uart());

    while serial.read() != Ok(0x00) {}

    init_logger(serial);

    info!("Initialized");

    let mut builtin_led = pins.gpio2.into_push_pull_output();
    let mut red_led = pins.gpio5.into_push_pull_output();
    let mut buzzer = pins.gpio4.into_push_pull_output();

    let mut button1 = pins.gpio14.into_pull_up_input();
    let mut button2 = pins.gpio12.into_pull_up_input();

    let (timer1, mut timer2) = dp.TIMER.timers();

    initialize_timekeeping(timer1);

    builtin_led.set_high().unwrap();
    red_led.set_high().unwrap();
    buzzer.set_low().unwrap();

    info!("Starting");

    for song in [THE_GOOD_LIFE, JERK_IT_OUT] {
        info!("Playing song...");
        for note in song {
            trace!(
                "{} {} for {}ms yield for {}ms",
                note.tone.letter,
                note.tone.octave,
                note.sustain,
                note.delay
            );

            let frequency = note.tone.frequency().round() as u32;
            let sustain_cycles = ((note.sustain as f32 / 1000.0) * frequency as f32).round() as u32;

            trace!("{}Hz for {} cycles", frequency, sustain_cycles);

            for _ in 0..sustain_cycles {
                buzzer.set_high().unwrap();
                red_led.set_high().unwrap();
                timer2.delay_us(1_000_000 / (frequency * 2));

                buzzer.set_low().unwrap();
                red_led.set_low().unwrap();
                timer2.delay_us(1_000_000 / (frequency * 2));
            }

            timer2.delay_ms(note.delay);
        }

        timer2.delay_ms(2000);
    }

    info!("Finished");

    loop {
        hint::spin_loop()
    }
}
