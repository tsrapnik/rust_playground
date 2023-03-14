use stm32f4xx_hal::{pac, prelude::*, gpio::PinState};

fn main() {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        let gpioa = dp.GPIOA.split();
        let mut led = gpioa.pa10.into_push_pull_output_in_state(PinState::High);

        led.set_low();
        let mut ana = led.into_analog();
        let adc = dp.ADC1;
    }
}
