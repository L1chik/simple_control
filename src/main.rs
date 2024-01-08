#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]


use defmt::*;
use embassy_executor::{Spawner, task};
use embassy_stm32::adc::Adc;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_time::{Timer, Delay, Duration};
use {defmt_rtt as _, panic_probe as _};

#[task]
async fn blink(mut led: Output<'static, AnyPin>) {
    loop {
        led.toggle();
        Timer::after(Duration::from_millis(800)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // let mut delay = Delay;
    // let mut adc = Adc::new(p.ADC1, &mut delay);
    // let mut x_axis = p.PA1;
    // let mut y_axis = p.PA0;
    //
    //
    //
    // let mut vrefint = adc.enable_vrefint();
    //
    // let vrefint_sample = adc.read(&mut vrefint);

    let green = Output::new(p.PA5, Level::High , Speed::Low).degrade();
    spawner.spawn(blink(green)).unwrap();
    // info!("VrefInt: {}", vrefint_sample);

    let mut step = Output::new(p.PA1, Level::High, Speed::Low);
    let mut dir = Output::new(p.PA0, Level::High, Speed::Low);
    // let mut en = Output::new(p.PA4, Level::Low, Speed::Medium);
    let ps: u64 = 200;

    loop {
        // let x = adc.read(&mut x_axis);
        // let y = adc.read(&mut y_axis);
        // info!("X: {}, Y: {})", x, y);
        // en.set_high();
        dir.set_high();
        for i in 0..4000 {
            info!("step forward");

            step.set_low();
            // Delay::delay_us(500);
            Timer::after(Duration::from_micros(ps)).await;
            step.set_high();
            // Delay::delay_us(500);
            Timer::after(Duration::from_micros(ps)).await;
        }
        Timer::after(Duration::from_millis(5000)).await;


        dir.set_low();
        for i in 0..8000 {
            info!("step backward");

            step.set_high();
            Timer::after(Duration::from_micros(ps)).await;
            step.set_low();
            Timer::after(Duration::from_micros(ps)).await;
        }

        Timer::after(Duration::from_millis(5000)).await;

    }
}