#![no_std]
#![no_main]

use cortex_m_rt::exception;
use stm32l0::stm32l0x3;

use panic_halt as _;

fn enable_hsi(per: &stm32l0x3::Peripherals) {
    // enable hsi16
    per.RCC.cr.modify(|_, w| w.hsi16on().enabled());
    while per.RCC.cr.read().hsi16rdyf().is_not_ready() {}

    // set system clock to hsi16
    per.RCC.cfgr.write(|w| w.sw().hsi16());
}

static mut MILLIS: u32 = 0;

fn run() -> ! {
    let per = stm32l0x3::Peripherals::take().unwrap();

    enable_hsi(&per);

    per.STK
        .rvr
        .write(|w| unsafe { w.reload().bits((16000000 / 1000) as u32 - 1) });
    per.STK.cvr.write(|w| unsafe { w.current().bits(0) });
    per.STK.csr.write(|w| {
        w.tickint()
            .set_bit()
            .clksource()
            .set_bit()
            .enable()
            .set_bit()
    });

    per.RCC.iopenr.write(|w| w.iopaen().set_bit());
    per.GPIOA.moder.write(|w| w.mode5().output());

    loop {
        if (unsafe { MILLIS } / 1000) % 2 == 0 {
            per.GPIOA.odr.write(|w| w.od5().high());
        } else {
            per.GPIOA.odr.write(|w| w.od5().low());
        }
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    run()
}

#[exception]
fn SysTick() {
    unsafe {
        MILLIS += 1;
    }
}
