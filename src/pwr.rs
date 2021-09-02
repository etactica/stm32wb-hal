/// Enables or disables USB power supply.
pub fn set_usb(enable: bool) {
    let pwr = unsafe { &*stm32wb_pac::PWR::ptr() };
    pwr.cr2.modify(|_, w| w.usv().bit(enable));
}

/// Enables or disables CPU2 Cortex-M0 radio co-processor.
pub fn set_cpu2(enabled: bool) {
    let pwr = unsafe { &*stm32wb_pac::PWR::ptr() };
    pwr.cr4.modify(|_, w| w.c2boot().bit(enabled))
}

/// Enables or disables access to the backup domain.
pub fn set_backup_access(enabled: bool) {
    let pwr = unsafe { &*stm32wb_pac::PWR::ptr() };

    // ST: write twice the value to flush the APB-AHB bridge to ensure the bit is written
    pwr.cr1.modify(|_, w| w.dbp().bit(enabled));
    pwr.cr1.modify(|_, w| w.dbp().bit(enabled));
}

/// Set Low-Power mode for CPU2
pub fn set_cpu2_lpmode(mode: Cpu2LowPowerMode) {
    let pwr = unsafe { &*stm32wb_pac::PWR::ptr() };
    pwr.c2cr1.modify(|_, w| unsafe {w.lpms().bits(mode as u8)});
}

pub fn set_voltage_range(range: VoltageRange) {
    let pwr = unsafe { &*stm32wb_pac::PWR::ptr() };
    pwr.cr1.modify(|_, w| unsafe {w.vos().bits(range as u8)});
}

#[derive(Debug, Copy, Clone)]
pub enum Cpu2LowPowerMode {
    Stop0 = 0b000,
    Stop1 = 0b001,
    Stop2 = 0b010,
    Standby = 0b011,
    Shutdown = 0b100,
}

#[derive(Debug, Copy, Clone)]
pub enum VoltageRange {
    Range1 = 0b01,
    Range2 = 0b10,
}
