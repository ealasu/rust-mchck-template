pub mod port;
pub mod pt;
pub mod sim;
pub mod uart0;
pub mod uart;
pub mod usb;

macro_rules! reg {
    ($fn_name:ident, $ty:ty, $addr:expr) => {
        #[inline(always)]
        pub fn $fn_name() -> &'static mut $ty {
            unsafe { &mut *($addr as *mut $ty) }
        }
    }
}

//const FTFL_FlashConfig: usize = 0x00000400;
//const DMA: usize = 0x40008000;
//const FMC: usize = 0x4001f000;
//const FTFL: usize = 0x40020000;
//const DMAMUX: usize = 0x40021000;
//const SPI0: usize = 0x4002c000;
//const I2S0: usize = 0x4002f000;
//const CRC: usize = 0x40032000;
//const USBDCD: usize = 0x40035000;
//const PDB0: usize = 0x40036000;
//const PIT: usize = 0x40037000;
//const FTM0: usize = 0x40038000;
//const FTM1: usize = 0x40039000;
//const ADC0: usize = 0x4003b000;
//const RTC: usize = 0x4003d000;
//const RFVBAT: usize = 0x4003e000;
//const LPTMR0: usize = 0x40040000;
//const RFSYS: usize = 0x40041000;
//const TSI0: usize = 0x40045000;
reg!(sim, sim::Sim, 0x40047000);
reg!(porta, port::Port, 0x40049000);
reg!(portb, port::Port, 0x4004a000);
reg!(portc, port::Port, 0x4004b000);
reg!(portd, port::Port, 0x4004c000);
reg!(porte, port::Port, 0x4004d000);
//const WDOG: usize = 0x40052000;
//const EWM: usize = 0x40061000;
//const CMT: usize = 0x40062000;
//const MCG: usize = 0x40064000;
//const OSC0: usize = 0x40065000;
//const I2C0: usize = 0x40066000;
reg!(uart0, uart0::Uart0, 0x4006a000);
reg!(uart1, uart::Uart, 0x4006b000);
reg!(uart2, uart::Uart, 0x4006c000);
reg!(usb0, usb::Usb, 0x40072000);
//const CMP0: usize = 0x40073000;
//const CMP1: usize = 0x40073008;
//const VREF: usize = 0x40074000;
//const LLWU: usize = 0x4007c000;
//const PMC: usize = 0x4007d000;
//const SMC: usize = 0x4007e000;
//const RCM: usize = 0x4007f000;
reg!(pta, pt::Pt, 0x400ff000);
reg!(ptb, pt::Pt, 0x400ff040);
reg!(ptc, pt::Pt, 0x400ff080);
reg!(ptd, pt::Pt, 0x400ff0c0);
reg!(pte, pt::Pt, 0x400ff100);
