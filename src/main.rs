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

    let serial = dp
        .UART0
        .serial(pins.gpio1.into_uart(), pins.gpio3.into_uart());

    init_logger(serial);

    info!("Initialized");

    // write!(serial, "\r\nInitialized:\r\n").unwrap();

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

        trace!(
            "freq_secs: {}us, sustain_cycles: {}",
            freq_secs.0,
            sustain_cycles
        );

        for _ in 0..sustain_cycles {
            // buzzer.set_high().unwrap();
            red_led.set_high().unwrap();
            timer2.delay_us(freq_secs.0 / 2);

            // buzzer.set_low().unwrap();
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

type JerkItOutNote = MidiNote<Hertz, Milliseconds, Milliseconds>;
static JERK_IT_OUT: &[JerkItOutNote] = &[
    JerkItOutNote {
        freq: Hertz(415),
        sustain: Milliseconds(500),
        delay: Milliseconds(0),
    },
    JerkItOutNote {
        freq: Hertz(493),
        sustain: Milliseconds(250),
        delay: Milliseconds(0),
    },
    JerkItOutNote {
        freq: Hertz(415),
        sustain: Milliseconds(250),
        delay: Milliseconds(0),
    },
    JerkItOutNote {
        freq: Hertz(493),
        sustain: Milliseconds(250),
        delay: Milliseconds(0),
    },
    JerkItOutNote {
        freq: Hertz(493),
        sustain: Milliseconds(250),
        delay: Milliseconds(0),
    },
    JerkItOutNote {
        freq: Hertz(415),
        sustain: Milliseconds(500),
        delay: Milliseconds(0),
    },
    JerkItOutNote {
        freq: Hertz(370),
        sustain: Milliseconds(500),
        delay: Milliseconds(0),
    },
    JerkItOutNote {
        freq: Hertz(466),
        sustain: Milliseconds(250),
        delay: Milliseconds(0),
    },
    JerkItOutNote {
        freq: Hertz(370),
        sustain: Milliseconds(250),
        delay: Milliseconds(0),
    },
    JerkItOutNote {
        freq: Hertz(330),
        sustain: Milliseconds(200),
        delay: Milliseconds(0),
    },
    JerkItOutNote {
        freq: Hertz(415),
        sustain: Milliseconds(200),
        delay: Milliseconds(50),
    },
    JerkItOutNote {
        freq: Hertz(415),
        sustain: Milliseconds(200),
        delay: Milliseconds(0),
    },
];

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
            "Note @ {}Hz for {}us with a delay of {}us",
            self.freq.into().0,
            self.sustain.into().0,
            self.delay.into().0
        );
    }
}

impl<F, S, D> MidiNote<F, S, D>
where
    F: Into<Hertz> + Clone + Copy,
    S: Into<Microseconds> + Clone + Copy,
    D: Into<Microseconds> + Clone + Copy,
{
    pub fn normalize(self: &MidiNote<F, S, D>) -> MidiNote<Hertz, Microseconds, Microseconds> {
        MidiNote {
            freq: self.freq.into(),
            sustain: self.sustain.into(),
            delay: self.delay.into(),
        }
    }
}
