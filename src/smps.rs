//! Switch-Mode Power Supply (SMPS) module

pub struct Smps {}

impl Smps {
    pub fn enable() {
        let pwr = unsafe { stm32wb_pac::Peripherals::steal().PWR };
        pwr.cr5.modify(|_, w| w.sdeb().set_bit())
    }

    pub fn disable() {
        let pwr = unsafe { stm32wb_pac::Peripherals::steal().PWR };
        pwr.cr5.modify(|_, w| w.sdeb().clear_bit())
    }

    pub fn is_enabled() -> bool {
        let pwr = unsafe { stm32wb_pac::Peripherals::steal().PWR };
        pwr.cr5.read().sdeb().bit()
    }

    pub fn startup_current(current: SmpsStartupCurrent) {
        let pwr = unsafe { stm32wb_pac::Peripherals::steal().PWR };
        pwr.cr5.modify(|_, w| unsafe { w.sdsc().bits(current as u8) })
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SmpsStartupCurrent {
    I80mA = 0b000,
    I100mA = 0b001,
    I120mA = 0b010,
    I140mA = 0b011,
    I160mA = 0b100,
    I180mA = 0b101,
    I200mA = 0b110,
    I220mA = 0b111,
}
