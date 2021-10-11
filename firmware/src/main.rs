#![no_std]
#![no_main]

use core::{
    hint, panic,
    sync::atomic::{self, Ordering},
};
use defmt::{error, info, trace, Format};
use esp8266_hal::{
    gpio::*,
    prelude::*,
    target::Peripherals,
    time::{Hertz, Microseconds, Milliseconds},
};

use crate::{
    logger::{init_logger, PanicInfo},
    time::initialize_timekeeping,
};

mod logger;
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

    for note in JERK_IT_OUT {
        trace!("{:?}", note);

        let MidiNote {
            freq,
            sustain,
            delay,
        } = note.normalize();

        let freq_secs = Microseconds::from(freq);
        let sustain_cycles = sustain.0 / freq_secs.0;

        for _ in 0..sustain_cycles {
            buzzer.set_high().unwrap();
            red_led.set_high().unwrap();
            timer2.delay_us(freq_secs.0 / 2);

            buzzer.set_low().unwrap();
            red_led.set_low().unwrap();
            timer2.delay_us(freq_secs.0 / 2);
        }

        timer2.delay_us(delay.0);
    }

    info!("Finished");

    loop {
        hint::spin_loop()
    }
}

type MegalovaniaNote = MidiNote<Hertz, Milliseconds, Milliseconds>;
static MEGALOVANIA: &[MegalovaniaNote] = &[
    MidiNote {
        freq: Hertz(293),
        sustain: Milliseconds(50),
        delay: Milliseconds(50),
    },
    MidiNote {
        freq: Hertz(293),
        sustain: Milliseconds(100),
        delay: Milliseconds(0),
    },
    MidiNote {
        freq: Hertz(587),
        sustain: Milliseconds(100),
        delay: Milliseconds(100),
    },
    MidiNote {
        freq: Hertz(440),
        sustain: Milliseconds(200),
        delay: Milliseconds(100),
    },
    MidiNote {
        freq: Hertz(415),
        sustain: Milliseconds(100),
        delay: Milliseconds(100),
    },
    MidiNote {
        freq: Hertz(392),
        sustain: Milliseconds(100),
        delay: Milliseconds(100),
    },
    MidiNote {
        freq: Hertz(349),
        sustain: Milliseconds(200),
        delay: Milliseconds(0),
    },
    MidiNote {
        freq: Hertz(293),
        sustain: Milliseconds(100),
        delay: Milliseconds(0),
    },
    MidiNote {
        freq: Hertz(349),
        sustain: Milliseconds(100),
        delay: Milliseconds(0),
    },
    MidiNote {
        freq: Hertz(392),
        sustain: Milliseconds(100),
        delay: Milliseconds(0),
    },
];

macro_rules! notes {
    ((
        frequency = $freq_ty:ident,
        time = $time_ty:ident
    )[
        $($freq:literal for $sustain:literal $(yield for $delay:literal)?;)+
    ]) => {
        &[
            $(
                &MidiNote {
                    freq: $freq_ty($freq),
                    sustain: $time_ty($sustain),
                    delay: $time_ty(0 $(+ $delay)?)
                }
            ),+
        ]
    };
}

static JERK_IT_OUT: &[&dyn Note] = notes! {
    (
        frequency = Hertz,
        time = Milliseconds
    )
    [
        415 for 500;
        493 for 250;
        415 for 250;
        493 for 200 yield for 50;
        493 for 200 yield for 50;
        415 for 500;
        370 for 500;
        466 for 250;
        370 for 250;
        330 for 500 yield for 1000;
        // 415 for 200 yield for 50;
        // 415 for 200;
    ]
};

#[derive(Clone, Copy)]
struct MidiNote<F, S, D>
where
    F: Into<Hertz> + Clone + Copy,
    S: Into<Microseconds> + Clone + Copy,
    D: Into<Microseconds> + Clone + Copy,
{
    freq: F,
    sustain: S,
    delay: D,
}

impl<F, S, D> Format for MidiNote<F, S, D>
where
    F: Into<Hertz> + Clone + Copy,
    S: Into<Microseconds> + Clone + Copy,
    D: Into<Microseconds> + Clone + Copy,
{
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "{}Hz for {}ms yield for {}ms",
            self.freq.into().0,
            self.sustain.into().0 as f64/ 1000.0,
            self.delay.into().0 as f64 / 1000.0
        );
    }
}

trait Note: Format + Send + Sync {
    fn normalize(&self) -> MidiNote<Hertz, Microseconds, Microseconds>;
}

impl<F, S, D> Note for MidiNote<F, S, D>
where
    F: Into<Hertz> + Clone + Copy + Send + Sync,
    S: Into<Microseconds> + Clone + Copy + Send + Sync,
    D: Into<Microseconds> + Clone + Copy + Send + Sync,
{
    fn normalize(&self) -> MidiNote<Hertz, Microseconds, Microseconds> {
        MidiNote {
            freq: self.freq.into(),
            sustain: self.sustain.into(),
            delay: self.delay.into(),
        }
    }
}
