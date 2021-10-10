#![no_std]
#![no_main]

use core::{
    fmt::Write,
    mem::{self, MaybeUninit},
    panic::PanicInfo,
};
use esp8266_hal::{
    gpio::*,
    interrupt::{enable_interrupt, InterruptType},
    prelude::*,
    target::Peripherals,
    time::{Hertz, Microseconds, Milliseconds, Nanoseconds},
    uart::UART0Serial,
    watchdog::Watchdog,
};
// use panic_halt as _;
use xtensa_lx::mutex::{CriticalSectionMutex, Mutex};

use crate::time::{current_millis, initialize_timekeeping};

mod time;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    (&SERIAL).lock(|serial| {
        serial
            .as_mut()
            .and_then(|serial| writeln!(serial, "PANIC: {}", info).ok())
    });

    loop {
        xtensa_lx::debug_break();
    }
}

#[entry]
fn start() -> ! {
    main();
}

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

    let (timer1, mut timer2) = dp.TIMER.timers();

    initialize_timekeeping(timer1);

    builtin_led.set_high().unwrap();
    red_led.set_high().unwrap();
    buzzer.set_low().unwrap();

    (&SERIAL).lock(|serial_lock| *serial_lock = Some(serial));

    (&SERIAL).lock(|serial| {
        serial
            .as_mut()
            .map(|serial| write!(serial, "Starting\n\r").unwrap())
    });

    for note in JERK_IT_OUT {
        (&SERIAL).lock(|serial| {
            serial
                .as_mut()
                .map(|serial| writeln!(serial, "{:?}", note).unwrap())
        });

        let MidiNote {
            freq,
            sustain,
            delay,
        } = note.normalize();

        let freq_secs = Microseconds::from(freq);
        let sustain_cycles = sustain.0 / freq_secs.0;

        (&SERIAL).lock(|serial| {
            serial.as_mut().map(|serial| {
                writeln!(
                    serial,
                    "freq_secs: {:?}, sustain_cycles: {:?}",
                    freq_secs, sustain_cycles
                )
                .unwrap()
            })
        });

        for _ in 0..sustain_cycles {
            timer2.delay_us(Microseconds::from(freq).0);
            builtin_led.toggle().unwrap();
            buzzer.toggle().unwrap();
        }

        timer2.delay_us(delay.0);
    }

    loop {
        (&SERIAL)
            .lock(|serial| serial.as_mut().map(|serial| writeln!(serial, "Finished")))
            .transpose()
            .unwrap();
        xtensa_lx::debug_break();
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
        sustain: Milliseconds(500),
        delay: Milliseconds(0),
    },
];

// First loop
//  { .f = 293, .s = 50, .d = 100 },
//  { .f = 293, .s = 0, .d = 100 },
//  { .f = 587, .s = 100, .d = 200 },
//  { .f = 440, .s = 200, .d = 300 },
//  { .f = 415, .s = 100, .d = 200 },
//  { .f = 392, .s = 100, .d = 200 },
//  { .f = 349, .s = 0, .d = 200 },
//  { .f = 293, .s = 0, .d = 100 },
//  { .f = 349, .s = 0, .d = 100 },
//  { .f = 392, .s = 0, .d = 100 },

#[derive(Debug, Clone, Copy)]
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
