use super::config::*;

/// PLL input frequency source.
#[derive(Debug, Clone)]
pub enum PllSrc {
    Msi(MsiRange),
    Hsi,
    Hse(HseDivider),
}

/// System clock (SYSCLK) source selection.
#[derive(Debug, Clone)]
pub enum SysClkSrc {
    /// Multi-speed internal RC oscillator
    Msi(MsiRange),

    /// 16 MHz internal RC
    Hsi,

    /// Use HSE directly, without PLL.
    HseSys(HseDivider),

    /// Use PLL.
    Pll(PllSrc),
}

/// USB (48 MHz) clock source selection.
#[derive(Debug, Copy, Clone)]
pub enum UsbClkSrc {
    Hsi48 = 0b00,
    PllSai1Q = 0b01,
    PllQ = 0b10,
    Msi = 0b11,
}

impl Default for UsbClkSrc {
    fn default() -> Self {
        UsbClkSrc::PllSai1Q
    }
}


/// SMPS clock source, note, that when the radio is active, this is hard set to HSE
#[derive(Debug, Copy, Clone)]
pub enum SmpsClkSrc {
    Hsi16 = 0b00,
    Msi = 0b01,
    Hse = 0b10,
}

impl Default for SmpsClkSrc {
    /// The default is MSI after POR, but HSI16 from standby!
    fn default() -> Self {
        SmpsClkSrc::Msi
    }
}
