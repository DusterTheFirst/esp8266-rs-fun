use esp8266_hal::{
    interrupt::{enable_interrupt, InterruptType},
    prelude::*,
    timer::Timer1,
};
use xtensa_lx::mutex::{CriticalSectionMutex, Mutex};

static CURRENT_MILLIS: CriticalSectionMutex<u128> = CriticalSectionMutex::new(0);

pub fn initialize_timekeeping(mut timer: Timer1) {
    enable_interrupt(InterruptType::TIMER1);

    timer.enable_interrupts();
    timer.start(1.ms());
}

#[interrupt]
fn timer1() {
    (&CURRENT_MILLIS).lock(|millis| *millis += 1);
}

pub fn current_millis() -> u128 {
    (&CURRENT_MILLIS).lock(|millis| *millis)
}
