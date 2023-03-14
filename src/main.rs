use stm32f4::stm32f401;

fn main() {
    let peripherals = stm32f401::Peripherals::take().unwrap();
    let gpioa = peripherals.GPIOA;
    gpioa.odr.modify(|_, w| w.odr0().set_bit());
}
