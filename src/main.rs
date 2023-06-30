#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Rtc, IO};



#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;
    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();
    println!("Hello world!");

    let io = IO::new(peripherals.GPIO,peripherals.IO_MUX);
    let mut led12 = io.pins.gpio12.into_push_pull_output();
    let mut led13 = io.pins.gpio13.into_push_pull_output();
    led12.set_high().unwrap();
    led13.set_low().unwrap();


    let a = 1;
    let b = 1;
    let c =  delay(a,b);
    println!("Hello world!{}",c);

    loop {

    }
}

fn delay(a:i32,b:i32) -> i32{
/*
    asm!(
    "mov edi, ebx",
    "cpuid",
    "xchg edi, ebx",
    in("eax") info,
    lateout("eax") a, out("edi") b, out("ecx") c, out("edx") d,
    )*/

    a + b
}