use core::sync::atomic::{AtomicU32, Ordering};

use defmt::error;
use esp8266_hal::{
    interrupt::{enable_interrupt, InterruptType},
    prelude::*,
    timer::Timer1,
};

defmt::timestamp!("{=u128:µs}", current_millis() * 1000);

/// LSB representation of an AtomicU128 using AtomicU32s.
///
/// \[0\]: Least significant word \
/// \[3\]: Most significant word
static CURRENT_MILLIS: [AtomicU32; 4] = [
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
];

pub fn initialize_timekeeping(mut timer: Timer1) {
    enable_interrupt(InterruptType::TIMER1);

    timer.enable_interrupts();
    timer.start(1.ms());
}

#[interrupt]
fn timer1() {
    time_tick();
}

fn time_tick() {
    let mut carry = true;

    for word in &CURRENT_MILLIS {
        let old = word.load(Ordering::SeqCst);

        word.store(old + if carry { 1 } else { 0 }, Ordering::SeqCst);

        carry = old == u32::MAX;
    }

    if carry {
        error!("Timer wrapped!!! the device has either been running for way too long or something is super wrong");
    }
}

pub fn current_millis() -> u128 {
    CURRENT_MILLIS
        .iter()
        .enumerate()
        .fold(0u128, |previous, (i, word)| {
            previous | (word.load(Ordering::SeqCst) as u128) << i * 32
        })
}
