use embedded_graphics::Drawable;
use embedded_graphics::image::{Image, ImageRaw, ImageRawLE};
use embedded_graphics::mono_font::ascii::FONT_6X9;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::{BinaryColor, Rgb565};

use embedded_graphics::prelude::{Point, Primitive, RgbColor};
use embedded_graphics::primitives::{PrimitiveStyle, Triangle};
use embedded_graphics::text::Text;
use hal::ehal::blocking::delay::DelayMs;
use hal::ehal::blocking::spi::Write;
use hal::ehal::digital::v2::{InputPin, OutputPin};
use st7735_lcd::Orientation;
use embedded_graphics::prelude::*;


pub struct Lcd1in8<'a,SPI, CS, DC, RST, DELAY>{
    pub  lcd :st7735_lcd::ST7735<SPI, DC, RST>,
    delay: &'a mut DELAY,
    spi:&'a mut SPI
}


impl<'a,SPI, CS,  DC, RST, DELAY> Lcd1in8<'a,SPI, CS, DC, RST, DELAY>
    where
        SPI: Write<u8>,
        CS: OutputPin,
        DC: OutputPin,
        RST: OutputPin,
        DELAY: DelayMs<u8>,
{
    pub fn new(
        spi: &'a mut SPI,
        cs: CS,
        dc: DC,
        rgb: bool,
        inverted: bool,
        width: u32,
        height: u32,
        rst: RST,
        delay: &'a mut DELAY,
    ) -> Result<Self, SPI::Error> {
        let mut lcd = st7735_lcd::ST7735::new(spi, dc, rst, rgb, inverted, width, height);
        lcd.init(delay).unwrap();
        match lcd {
            Ok(v) => {
                let temp = Self { lcd: v, delay, spi };
                Ok(temp)
            },
            Err(e) => {
                Err(e)
            }
        }
    }
    pub  fn work(&mut self){

        self.lcd.clear(Default::default()).unwrap();
      /*  display
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
        text.draw(&mut display).expect("");*/
    }
}