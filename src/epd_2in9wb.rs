use core::fmt::Debug;
use esp_println::println;
use hal::ehal::blocking::delay::DelayMs;

use hal::ehal::blocking::spi::Write;
use hal::ehal::digital::v2::{InputPin, OutputPin};
use epd_waveshare::{epd2in9::*, graphics::DisplayRotation, prelude::*};
use embedded_graphics::{
    pixelcolor::BinaryColor::On as Black,
    pixelcolor::BinaryColor::{Off as White},

};
use epd_waveshare::prelude::Display;
use embedded_graphics::mono_font::{ MonoTextStyleBuilder};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line, PrimitiveStyle};
use embedded_graphics::text::{Baseline, Text, TextStyleBuilder};

pub struct Epd2in9wb<'a,SPI, CS, BUSY, DC, RST, DELAY>{
    pub  epd :Epd2in9<SPI, CS, BUSY, DC, RST, DELAY>,
    delay: &'a mut DELAY,
    spi:&'a mut SPI
}
impl<'a,SPI, CS, BUSY, DC, RST, DELAY> Epd2in9wb<'a,SPI, CS, BUSY, DC, RST, DELAY>
    where
        SPI: Write<u8>,
        CS: OutputPin,
        BUSY: InputPin,
        DC: OutputPin,
        RST: OutputPin,
        DELAY: DelayMs<u8>,
{

    pub fn new(
        spi: &'a mut SPI,
        cs: CS,
        busy: BUSY,
        dc: DC,
        rst: RST,
        delay: &'a mut DELAY,
    ) -> Result<Self, SPI::Error> {
        let mut epd = Epd2in9::new(spi, cs, busy, dc, rst,delay);

        match epd {
            Ok(v) => {
                let temp = Self{ epd: v,delay, spi };
                Ok(temp)
            },
            Err(e)=>{
                Err(e)
            }
        }

    }

    pub  fn work(&mut self){
        println!("Init done!");

        let mut display = Display2in9::default();
        println!("Drawing rotated text...");
        display.set_rotation(DisplayRotation::Rotate270);
        Self::draw_text(&mut display, "Rotate 0!1112222你好", 50, 50);
        let _ = Line::new(Point::new(10, 20), Point::new(100, 20))
            .into_styled(PrimitiveStyle::with_stroke(Black, 10))

            .draw(&mut display);
        let _= self.epd.clear_frame(self.spi, self.delay);

        let _= self.epd.update_frame(self.spi, &display.buffer(), self.delay);
        let _= self.epd.display_frame(self.spi, self.delay);


        let _= self.epd.sleep(self.spi, self.delay);
    }

    fn draw_text(display: &mut Display2in9, text: &str, x: i32, y: i32) {

        let style = MonoTextStyleBuilder::new()
            .font(&embedded_graphics::mono_font::iso_8859_16::FONT_10X20)
            .text_color(Black)
            .background_color(White)
            .build();

        let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

        let _ = Text::with_text_style(text, Point::new(x, y), style, text_style).draw(display);
    }

}
