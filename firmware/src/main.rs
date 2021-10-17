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
    music::THE_GOOD_LIFE,
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
    let mut red_led = pins.gpio13.into_push_pull_output();

    let mut buzzer_l = pins.gpio5.into_push_pull_output();
    let mut buzzer_r = pins.gpio4.into_push_pull_output();

    // let mut button1 = pins.gpio14.into_pull_up_input();
    // let mut button2 = pins.gpio12.into_pull_up_input();

    let (timer1, mut timer2) = dp.TIMER.timers();

    initialize_timekeeping(timer1);

    builtin_led.set_high().unwrap();
    red_led.set_high().unwrap();

    buzzer_l.set_low().unwrap();
    buzzer_r.set_low().unwrap();

    info!("Starting");

    for music in [THE_GOOD_LIFE] {
        info!("Playing \"{=str}\" @ {=u32} bpm", music.title, music.bpm);

        for note in music.track_1 {
            let frequency = note.frequency().round() as u32;
            let note_sustain_secs = (note.sustain / music.bpm) * 60;

            let sustain_cycles = note_sustain_secs * frequency;
            let sustain_cycles = sustain_cycles.numer() / sustain_cycles.denom();

            trace!(
                "{}{=u32} ({=u32}Hz) for {=u32}/{=u32}s ({=u32} cycles)",
                note.letter,
                note.octave,
                frequency,
                note_sustain_secs.numer(),
                note_sustain_secs.denom(),
                sustain_cycles,
            );

            red_led.set_high().unwrap();
            for _ in 0..sustain_cycles {
                buzzer_l.set_high().unwrap();
                timer2.delay_us(1_000_000 / (frequency * 2));

                buzzer_l.set_low().unwrap();
                timer2.delay_us(1_000_000 / (frequency * 2));
            }
            red_led.set_low().unwrap();

            let rest_sec = (note.rest / music.bpm) * 60;

            if rest_sec.numer() != &0 {
                trace!(
                    "rest for {=u32}/{=u32}ms",
                    rest_sec.numer(),
                    rest_sec.denom()
                );

                let rest_us = rest_sec * 1_000_000;
                let rest_us = rest_us.numer() / rest_us.denom();

                timer2.delay_us(rest_us);
            }
        }

        timer2.delay_ms(2000);
    }

    info!("Finished");

    loop {
        hint::spin_loop()
    }
}
