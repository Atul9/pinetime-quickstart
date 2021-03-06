#![no_std]
#![no_main]

extern crate cortex_m_rt as rt; // v0.5.x

extern crate nrf52832_hal;
extern crate panic_halt;

use cortex_m_rt::entry;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;
use embedded_graphics::style::PrimitiveStyle;
use embedded_graphics::image::*;
use embedded_graphics::pixelcolor::Rgb565;
use nrf52832_hal::gpio::Level;
use nrf52832_hal::gpio::*;
use nrf52832_hal::spim;
use nrf52832_hal::Delay;
use st7789::{ST7789, Orientation};
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    let core = nrf52832_hal::nrf52832_pac::CorePeripherals::take().unwrap();
    let delay = Delay::new(core.SYST);

    let p = nrf52832_hal::nrf52832_pac::Peripherals::take().unwrap();
    let port0 = p.P0.split();

    let _backlight = port0.p0_22.into_push_pull_output(Level::Low); // set medium backlight on
    let rst = port0.p0_26.into_push_pull_output(Level::Low); // reset pin
    let _cs = port0.p0_25.into_push_pull_output(Level::Low); // keep low while drivign display
    let dc = port0.p0_18.into_push_pull_output(Level::Low); // data/clock switch

    let spiclk = port0.p0_02.into_push_pull_output(Level::Low).degrade(); // SPI clock to LCD
    let spimosi = port0.p0_03.into_push_pull_output(Level::Low).degrade(); // SPI MOSI to LCD

    let pins = spim::Pins {
        sck: spiclk,
        miso: None,
        mosi: Some(spimosi),
    };

    // create SPI interface
    let spi = spim::Spim::new(p.SPIM0, pins, spim::Frequency::M8, spim::MODE_3, 122);

    // create driver
    let mut display = ST7789::new(spi, dc, rst, 240, 240, delay);

    // initialize
    display.init().unwrap();
    // set default orientation
    display.set_orientation(&Orientation::Landscape).unwrap();

    let black = (0, 0, 0);
    
    let blank = Rectangle::new(Point::new(0, 0), Point::new(239, 239)).into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK));
    let raw_image_data = ImageRawLE::new(include_bytes!("../assets/ferris.raw"), 86, 64);
    let ferris = Image::new(&raw_image_data, Point::new(34, 8));

    // draw two circles on blue background
    blank.draw(&mut display).unwrap();
    ferris.draw(&mut display).unwrap();

    hprintln!("Rendering done").unwrap();

    loop {
        continue; // keep optimizer from removing in --release
    }
}
