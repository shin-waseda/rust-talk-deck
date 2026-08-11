use crate::controls::{FiveWaySwitch, TopButtons};
use panic_halt as _;
use wio_terminal as wio;
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;

pub type LcdDisplay = wio::LCD;

pub type AccelHandle = lis3dh::Lis3dh<
    wio::hal::sercom::i2c::I2c<
        wio::hal::sercom::i2c::Config<
            wio::hal::sercom::i2c::Pads<
                wio::hal::sercom::Sercom4,
                wio::hal::sercom::pad::IoSet3,
                wio::aliases::I2c0Sda,
                wio::aliases::I2c0Scl,
            >,
        >,
    >,
>;

pub struct Board {
    pub display: LcdDisplay,
    pub delay: Delay,
    pub five_way: FiveWaySwitch<
        wio::hal::gpio::Pin<wio::hal::gpio::PD08, wio::hal::gpio::Input<wio::hal::gpio::PullUp>>,
        wio::hal::gpio::Pin<wio::hal::gpio::PD09, wio::hal::gpio::Input<wio::hal::gpio::PullUp>>,
        wio::hal::gpio::Pin<wio::hal::gpio::PD10, wio::hal::gpio::Input<wio::hal::gpio::PullUp>>,
        wio::hal::gpio::Pin<wio::hal::gpio::PD20, wio::hal::gpio::Input<wio::hal::gpio::PullUp>>, // PD20 (Up)
        wio::hal::gpio::Pin<wio::hal::gpio::PD12, wio::hal::gpio::Input<wio::hal::gpio::PullUp>>, // PD12 (Left)
    >,
    pub top_buttons: TopButtons<
        wio::hal::gpio::Pin<wio::hal::gpio::PC26, wio::hal::gpio::Input<wio::hal::gpio::PullUp>>,
        wio::hal::gpio::Pin<wio::hal::gpio::PC27, wio::hal::gpio::Input<wio::hal::gpio::PullUp>>,
        wio::hal::gpio::Pin<wio::hal::gpio::PC28, wio::hal::gpio::Input<wio::hal::gpio::PullUp>>,
    >,
    pub accelerometer: AccelHandle, 
}

impl Board {
    pub fn init() -> Self {
        let mut peripherals = Peripherals::take().unwrap();
        let core = CorePeripherals::take().unwrap();

        let mut clocks = GenericClockController::with_external_32kosc(
            peripherals.gclk,
            &mut peripherals.mclk,
            &mut peripherals.osc32kctrl,
            &mut peripherals.oscctrl,
            &mut peripherals.nvmctrl,
        );
        let mut delay = Delay::new(core.SYST, &mut clocks);

        let sets = wio::Pins::new(peripherals.port).split();

        let (display, _backlight) = sets
            .display
            .init(
                &mut clocks,
                peripherals.sercom7,
                &mut peripherals.mclk,
                58.MHz(),
                &mut delay,
            )
            .unwrap();

        let five_way = FiveWaySwitch::new(
            sets.buttons.switch_x.into_pull_up_input(),
            sets.buttons.switch_y.into_pull_up_input(),
            sets.buttons.switch_z.into_pull_up_input(),
            sets.buttons.switch_u.into_pull_up_input(),
            sets.buttons.switch_b.into_pull_up_input(),
        );

        let top_buttons = TopButtons::new(
            sets.buttons.button1.into_pull_up_input(),
            sets.buttons.button2.into_pull_up_input(),
            sets.buttons.button3.into_pull_up_input(),
        );

        let accelerometer = sets
            .accelerometer
            .init(&mut clocks, peripherals.sercom4, &mut peripherals.mclk);

        Self {
            display,
            delay,
            five_way,
            top_buttons,
            accelerometer, 
        }
    }
}