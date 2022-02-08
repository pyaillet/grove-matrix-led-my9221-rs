#![no_main]
#![no_std]

use core::convert::TryInto;

use cortex_m_rt::entry;

use stm32f3_discovery::stm32f3xx_hal::{self as hal, pac, prelude::*};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    let cp = pac::CorePeripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    // Configure I2C1
    let mut scl =
        gpiob
            .pb6
            .into_af4_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    let mut sda =
        gpiob
            .pb7
            .into_af4_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    scl.internal_pull_up(&mut gpiob.pupdr, true);
    sda.internal_pull_up(&mut gpiob.pupdr, true);
    let i2c = hal::i2c::I2c::new(
        dp.I2C1,
        (scl, sda),
        40.kHz().try_into().unwrap(),
        clocks,
        &mut rcc.apb1,
    );

    let delay = hal::delay::Delay::new(cp.SYST, clocks);

    let mut led_matrix = grove_matrix_led_my9221_rs::My9221LedMatrix::new(
        grove_matrix_led_my9221_rs::DEFAULT_ADDRESS,
        i2c,
        delay,
    );

    let mut emoji_num = 0;

    const DELAY: u16 = 5_000u16;

    loop {
        led_matrix.display_emoji(emoji_num, DELAY, true);
        led_matrix.delay_ms(DELAY.into());
        emoji_num = (emoji_num + 1) % 32;
    }
}
