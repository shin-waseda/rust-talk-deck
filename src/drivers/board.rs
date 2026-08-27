use crate::drivers::controls::{FiveWaySwitch, TopButtons};
use panic_halt as _;
use wio_terminal as wio;
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;

use embedded_sdmmc::{TimeSource, Timestamp};

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

pub type SdCardHandle = wio::SDCardController<DummyTimesource>;

/// SDカードにはファイル更新日時を記録する仕組みが無いので、ダミーの時刻源を使う
pub struct DummyTimesource();
impl TimeSource for DummyTimesource {
    fn get_timestamp(&self) -> Timestamp {
        Timestamp { year_since_1970: 0, zero_indexed_month: 0, zero_indexed_day: 0, hours: 0, minutes: 0, seconds: 0 }
    }
}

/// SysTickを一切使わない、CPUサイクルカウントベースの軽量Delay。
/// SysTick本体はSDカード初期化に渡し切ってしまうため、メインループはこちらを使う。
pub struct CycleDelay {
    cpu_freq_hz: u32,
}

impl embedded_hal::blocking::delay::DelayMs<u16> for CycleDelay {
    fn delay_ms(&mut self, ms: u16) {
        let cycles = (ms as u64 * self.cpu_freq_hz as u64 / 1000) as u32;
        cortex_m::asm::delay(cycles.max(1));
    }
}

pub struct Board {
    pub display: LcdDisplay,
    pub delay: CycleDelay, // ★型がDelayからCycleDelayに変更
    pub five_way: FiveWaySwitch<
        wio::hal::gpio::Pin<wio::hal::gpio::PD08, wio::hal::gpio::Input<wio::hal::gpio::PullUp>>,
        wio::hal::gpio::Pin<wio::hal::gpio::PD09, wio::hal::gpio::Input<wio::hal::gpio::PullUp>>,
        wio::hal::gpio::Pin<wio::hal::gpio::PD10, wio::hal::gpio::Input<wio::hal::gpio::PullUp>>,
        wio::hal::gpio::Pin<wio::hal::gpio::PD20, wio::hal::gpio::Input<wio::hal::gpio::PullUp>>,
        wio::hal::gpio::Pin<wio::hal::gpio::PD12, wio::hal::gpio::Input<wio::hal::gpio::PullUp>>,
    >,
    pub top_buttons: TopButtons<
        wio::hal::gpio::Pin<wio::hal::gpio::PC26, wio::hal::gpio::Input<wio::hal::gpio::PullUp>>,
        wio::hal::gpio::Pin<wio::hal::gpio::PC27, wio::hal::gpio::Input<wio::hal::gpio::PullUp>>,
        wio::hal::gpio::Pin<wio::hal::gpio::PC28, wio::hal::gpio::Input<wio::hal::gpio::PullUp>>,
    >,
    pub accelerometer: AccelHandle,
    pub sd_card: SdCardHandle, // ★追加
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

        // SysTick本体。まずdisplay初期化に一時的に貸し、最後にSDカードへ渡し切る。
        let mut systick_delay = Delay::new(core.SYST, &mut clocks);

        let sets = wio::Pins::new(peripherals.port).split();

        let (display, _backlight) = sets
            .display
            .init(&mut clocks, peripherals.sercom7, &mut peripherals.mclk, 58.MHz(), &mut systick_delay)
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

        // SDカード初期化。ここでsystick_delayを消費し切る(以後SysTickは使えない)。
        let (sd_card, _sd_det) = sets
            .sd_card
            .init(&mut clocks, peripherals.sercom6, &mut peripherals.mclk, systick_delay, DummyTimesource())
            .expect("SD card init failed");

        // メインループの待機は、以後こちらのCPUサイクルベースDelayを使う
        let delay = CycleDelay { cpu_freq_hz: 120_000_000 };

        Self {
            display,
            delay,
            five_way,
            top_buttons,
            accelerometer,
            sd_card,
        }
    }
}