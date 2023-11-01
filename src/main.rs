#![no_std]
#![no_main]

/*
st7735 的最新版本 依赖的 embedded_graphics 0.8
epd_waveshare 的最新版本 依赖的  embedded_graphics 0.7.1

 */

mod cn_font;
pub(crate) mod epd_2in9wb;
pub(crate) mod lcd_1in8;
mod epd_3in27wb;

use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Rtc, IO, interrupt, gpio::{Event, Gpio0, Input, PullDown}, riscv, Delay, peripherals, esp_riscv_rt, TrapFrame};

use core::cell::RefCell;
use hal::riscv::_export::critical_section;
use critical_section::Mutex;
use embedded_graphics::image::{Image, ImageRaw, ImageRawLE};
use embedded_graphics::mono_font::ascii::FONT_6X9;
use embedded_graphics::mono_font::{MonoFont, MonoTextStyle, MonoTextStyleBuilder};
use embedded_graphics::pixelcolor::{BinaryColor, Rgb565};
use embedded_graphics::prelude::*;
use hal::gpio::{Gpio1, GpioPin, Unknown};


use epd_waveshare::{epd2in9::*, graphics::DisplayRotation, prelude::*};

use epd_waveshare::prelude::Display;

use epd_custom::epd3in27::{*};
use crate::epd_3in27wb::Epd3in27wb;

static BUTTON: Mutex<RefCell<Option<Gpio0<Input<PullDown>>>>> = Mutex::new(RefCell::new(None));
static BUTTON1: Mutex<RefCell<Option<Gpio1<Input<PullDown>>>>> = Mutex::new(RefCell::new(None));

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

    println!("entry_main!");

    let mut delay = Delay::new(&clocks);
    let io = IO::new(peripherals.GPIO,peripherals.IO_MUX);

    unsafe {
        riscv::interrupt::enable();
    }


    //墨水屏
    let epd_sclk = io.pins.gpio2;
    let epd_mosi = io.pins.gpio3;
    let epd_cs = io.pins.gpio7.into_push_pull_output();
    let epd_rst =io.pins.gpio10.into_push_pull_output();
    let epd_dc = io.pins.gpio6.into_push_pull_output();
    let mut spi = hal::Spi::new_no_cs_no_miso(
        peripherals.SPI2,
        epd_sclk,
        epd_mosi,
        32u32.MHz(),
        hal::spi::SpiMode::Mode0,
        &mut system.peripheral_clock_control,
        &clocks,
    );
    let busy_in = io.pins.gpio11.into_pull_up_input();

     let mut epd_device =epd_2in9wb::Epd2in9wb::new(&mut spi, epd_cs, busy_in, epd_dc, epd_rst,  &mut delay);
     epd_device.unwrap().work();
/*    let mut epd_device =Epd3in27wb::new(&mut spi, epd_cs, busy_in, epd_dc, epd_rst,  &mut delay);

    epd_device.unwrap().work();*/



    loop {
        /*  led2.toggle().unwrap();*/
        delay.delay_ms(500u32);
    }
}



#[interrupt]
fn GPIO(context: &mut esp_riscv_rt::TrapFrame) {
    critical_section::with(|cs| {
        println!("GPIO interrupt");
        println!("{:?}",context);



        let  button_is_high =  BUTTON
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .is_acore_interrupt_set();

        let  button1_is_high =  BUTTON1
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .is_acore_interrupt_set();

        if(button_is_high){
            println!("按钮0 按下");
        }

        if(button1_is_high){
            println!("按钮1 按下");
        }
        BUTTON
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt();

        BUTTON1
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt();

    });
}
