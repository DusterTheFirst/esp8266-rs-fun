use core::{panic, ptr::NonNull};

use defmt::Format;
use esp8266_hal::{prelude::_embedded_hal_serial_Write, uart::UART0Serial};
use xtensa_lx::mutex::{CriticalSectionMutex, Mutex};

#[defmt::global_logger]
struct Logger;

static mut IS_ACQUIRED: bool = false;

unsafe impl defmt::Logger for Logger {
    fn acquire() -> Option<NonNull<dyn defmt::Write>> {
        if unsafe { IS_ACQUIRED } {
            None
        } else {
            unsafe { IS_ACQUIRED = true }
            Some(NonNull::from(&Logger as &dyn defmt::Write))
        }
    }

    unsafe fn release(_writer: NonNull<dyn defmt::Write>) {
        IS_ACQUIRED = false
    }
}

impl defmt::Write for Logger {
    fn write(&mut self, bytes: &[u8]) {
        (&SERIAL).lock(|lock| {
            if let Some(serial) = lock {
                for &byte in bytes {
                    serial.write(byte).unwrap();
                }
            }
        });
    }
}

static SERIAL: CriticalSectionMutex<Option<UART0Serial>> = CriticalSectionMutex::new(None);

pub fn init_logger(serial: UART0Serial) {
    (&SERIAL).lock(|lock| lock.replace(serial));
}

pub struct PanicInfo<'p>(&'p panic::PanicInfo<'p>);

impl<'p> From<&'p panic::PanicInfo<'p>> for PanicInfo<'p> {
    fn from(info: &'p panic::PanicInfo<'p>) -> Self {
        PanicInfo(info)
    }
}

impl<'p> Format for PanicInfo<'p> {
    fn format(&self, fmt: defmt::Formatter) {
        let location = self.0.location().unwrap();
        let payload = self
            .0
            .payload()
            .downcast_ref::<&'static str>()
            .unwrap_or(&"");

        defmt::write!(
            fmt,
            "panicked at '{=str}', {=str}:{=u32}:{=u32}",
            payload,
            location.file(),
            location.line(),
            location.column()
        );
    }
}
