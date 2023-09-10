use core::fmt::Debug;
use esp_println::println;
use hal::ehal::blocking::delay::DelayMs;

use hal::ehal::blocking::spi::Write;
use hal::ehal::digital::v2::{InputPin, OutputPin};
use epd_waveshare::{ graphics::DisplayRotation, prelude::*};

use epd_waveshare::prelude::Display;
use embedded_graphics::mono_font::{ MonoTextStyleBuilder};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line, PrimitiveStyle};
use embedded_graphics::text::{Baseline, Text, TextStyleBuilder};
use epd_custom::epd3in27::{Display3in27, Epd3in27, TwoBitColorDisplay, TwoBitColor::White as White, TwoBitColor::Black as Black};
use epd_custom::epd3in27::TwoBitColor::{Gray1, Gray2};
use crate::epd_3in27wb;

pub struct Epd3in27wb<'a,SPI, CS, BUSY, DC, RST, DELAY>{
    pub  epd :Epd3in27<SPI, CS, BUSY, DC, RST, DELAY>,
    delay: &'a mut DELAY,
    spi:&'a mut SPI
}
impl<'a,SPI, CS, BUSY, DC, RST, DELAY> Epd3in27wb<'a,SPI, CS, BUSY, DC, RST, DELAY>
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
        let mut epd = Epd3in27::new(spi, cs, busy, dc, rst,delay);

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

        let mut display = Display3in27::default();
        println!("Drawing rotated text...");
        display.set_rotation(DisplayRotation::Rotate0);
        Self::draw_text(&mut display, "Rotate 0!1112222你好", 10, 10);
        let _ = Line::new(Point::new(10, 90), Point::new(90, 90))
            .into_styled(PrimitiveStyle::with_stroke(Black, 10))
            .draw(&mut display);

        let _ = Line::new(Point::new(10, 170), Point::new(90, 170))
            .into_styled(PrimitiveStyle::with_stroke(Gray2, 10))
            .draw(&mut display);


        let _ = Line::new(Point::new(10, 130), Point::new(90, 130))
            .into_styled(PrimitiveStyle::with_stroke(Gray1, 10))
            .draw(&mut display);
        let _= self.epd.clear_frame(self.spi, self.delay);

        let _= self.epd.update_frame(self.spi, &display.buffer(), self.delay);

        let _= self.epd.display_frame(self.spi, self.delay);
        println!("buffer长度：{}",display.buffer().len());


        let _= self.epd.sleep(self.spi, self.delay);
    }

    fn draw_text(display: &mut Display3in27, text: &str, x: i32, y: i32) {

        let style = MonoTextStyleBuilder::new()
            .font(&embedded_graphics::mono_font::iso_8859_16::FONT_10X20)
            .text_color(Black)
            .background_color(White)
            .build();

        let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

        let _ = Text::with_text_style(text, Point::new(x, y), style, text_style).draw(display);
    }

}
