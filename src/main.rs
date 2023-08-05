#![no_std]
#![no_main]



use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Rtc, IO, interrupt, gpio::{Event, Gpio0, Input, PullDown}, riscv, Delay, peripherals, esp_riscv_rt, TrapFrame};

use core::cell::RefCell;
use core::fmt::{Debug, Display};
use hal::riscv::_export::critical_section;
use critical_section::Mutex;
use embedded_graphics::image::{Image, ImageRaw, ImageRawLE};
use embedded_graphics::mono_font::ascii::FONT_6X9;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::{BinaryColor, Rgb565};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Triangle};
use embedded_graphics::text::Text;

use hal::gpio::{Gpio1, GpioPin, Unknown};
use st7735_lcd::Orientation;

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
    println!("Hello world!");

    let io = IO::new(peripherals.GPIO,peripherals.IO_MUX);
  /*  let mut led12 = io.pins.gpio12.into_push_pull_output();
    let mut led13 = io.pins.gpio13.into_push_pull_output();
    led12.set_low().unwrap();
    led13.set_low().unwrap();

    let mut led2 = io.pins.gpio2.into_push_pull_output();
    led2.set_high().unwrap();

    // Set GPIO0 as an input
     let mut button = io.pins.gpio0.into_pull_down_input();
     button.listen(Event::FallingEdge);

     critical_section::with(|cs| BUTTON.borrow_ref_mut(cs).replace(button));

     interrupt::enable(peripherals::Interrupt::GPIO, interrupt::Priority::Priority3).unwrap();

     let mut button1 = io.pins.gpio1.into_pull_down_input();
     button1.listen(Event::FallingEdge);

     critical_section::with(|cs| BUTTON1.borrow_ref_mut(cs).replace(button1));

     interrupt::enable(peripherals::Interrupt::GPIO, interrupt::Priority::Priority3).unwrap();*/

/*
    let aaa: GpioPin<Unknown, Bank0G, IRA, PINTYPE, SIG, GPIONUM> = io.pins.gpio12;

*/     unsafe {
         riscv::interrupt::enable();
     }

    let mut delay = Delay::new(&clocks);

    let sclk = io.pins.gpio2;
    let miso = io.pins.gpio0;
    let mosi = io.pins.gpio3;
    let cs = io.pins.gpio7;

    let mut spi = hal::Spi::new(
        peripherals.SPI2,
        sclk,
        mosi,
        miso,
        cs,
        32u32.MHz(),
        hal::spi::SpiMode::Mode0,
        &mut system.peripheral_clock_control,
        &clocks,
    );
    let rst =io.pins.gpio10.into_push_pull_output();
    let dc = io.pins.gpio6.into_push_pull_output();

    let rgb = true;
    let inverted = false;
    let width = 128;
    let height = 160;

    let mut display = st7735_lcd::ST7735::new(spi, dc, rst, rgb, inverted, width, height);

    display.init(&mut delay).unwrap();
    display.clear(Rgb565::BLACK).unwrap();
    display
        .set_orientation(&Orientation::Landscape)
        .unwrap();
    display.set_offset(0, 0);
    let yoffset = 100;
    let image_raw: ImageRawLE<Rgb565> =
        ImageRaw::new(include_bytes!("../assets/ferris.raw"), 86);
    let image = Image::new(&image_raw, Point::new(26, 8));
    image.draw(&mut display).unwrap();
    let thin_stroke = PrimitiveStyle::with_stroke(Rgb565::WHITE, 1);
    Triangle::new(
        Point::new(16, 16 + yoffset),
        Point::new(16 + 16, 16 + yoffset),
        Point::new(16 + 8, yoffset),
    )
        .into_styled(thin_stroke)
        .draw(&mut display).expect("绘制失败");

    let character_style = MonoTextStyle::new(&FONT_6X9, Rgb565::WHITE);

    let text = Text::new("Hello e-g", Point::new(10, 11), character_style);
    text.draw(&mut display).expect("");


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
