# [ doc = "System Integration Module" ]
# [ repr ( C ) ]
pub struct Sim {
    # [ doc = "0x00 - System Options Register 1" ]
    pub sopt1: Sopt1,
    # [ doc = "0x04 - SOPT1 Configuration Register" ]
    pub sopt1cfg: Sopt1cfg,
    _reserved0: [u8; 4092usize],
    # [ doc = "0x1004 - System Options Register 2" ]
    pub sopt2: Sopt2,
    _reserved1: [u8; 4usize],
    # [ doc = "0x100c - System Options Register 4" ]
    pub sopt4: Sopt4,
    # [ doc = "0x1010 - System Options Register 5" ]
    pub sopt5: Sopt5,
    _reserved2: [u8; 4usize],
    # [ doc = "0x1018 - System Options Register 7" ]
    pub sopt7: Sopt7,
    _reserved3: [u8; 8usize],
    # [ doc = "0x1024 - System Device Identification Register" ]
    pub sdid: Sdid,
    _reserved4: [u8; 12usize],
    # [ doc = "0x1034 - System Clock Gating Control Register 4" ]
    pub scgc4: Scgc4,
    # [ doc = "0x1038 - System Clock Gating Control Register 5" ]
    pub scgc5: Scgc5,
    # [ doc = "0x103c - System Clock Gating Control Register 6" ]
    pub scgc6: Scgc6,
    # [ doc = "0x1040 - System Clock Gating Control Register 7" ]
    pub scgc7: Scgc7,
    # [ doc = "0x1044 - System Clock Divider Register 1" ]
    pub clkdiv1: Clkdiv1,
    # [ doc = "0x1048 - System Clock Divider Register 2" ]
    pub clkdiv2: Clkdiv2,
    # [ doc = "0x104c - Flash Configuration Register 1" ]
    pub fcfg1: Fcfg1,
    # [ doc = "0x1050 - Flash Configuration Register 2" ]
    pub fcfg2: Fcfg2,
    # [ doc = "0x1054 - Unique Identification Register High" ]
    pub uidh: Uidh,
    # [ doc = "0x1058 - Unique Identification Register Mid-High" ]
    pub uidmh: Uidmh,
    # [ doc = "0x105c - Unique Identification Register Mid Low" ]
    pub uidml: Uidml,
    # [ doc = "0x1060 - Unique Identification Register Low" ]
    pub uidl: Uidl,
}

# [ repr ( C ) ]
pub struct Sopt1 {
    register: ::volatile_register::RW<u32>,
}

impl Sopt1 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Sopt1R, &'w mut Sopt1W) -> &'w mut Sopt1W
    {
        let bits = self.register.read();
        let r = Sopt1R { bits: bits };
        let mut w = Sopt1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Sopt1R {
        Sopt1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Sopt1W) -> &mut Sopt1W
    {
        let mut w = Sopt1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sopt1R {
    bits: u32,
}

impl Sopt1R {
    # [ doc = "Bits 12:15 - RAM size" ]
    pub fn ramsize(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 18:19 - 32K oscillator clock select" ]
    pub fn osc32ksel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 29 - USB voltage regulator in standby mode during VLPR and VLPW modes" ]
    pub fn usbvstby(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - USB voltage regulator in standby mode during Stop, VLPS, LLS and VLLS modes." ]
    pub fn usbsstby(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - USB voltage regulator enable" ]
    pub fn usbregen(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sopt1W {
    bits: u32,
}

impl Sopt1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Sopt1W { bits: 2147483648 }
    }
    # [ doc = "Bits 18:19 - 32K oscillator clock select" ]
    pub fn osc32ksel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - USB voltage regulator in standby mode during VLPR and VLPW modes" ]
    pub fn usbvstby(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - USB voltage regulator in standby mode during Stop, VLPS, LLS and VLLS modes." ]
    pub fn usbsstby(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - USB voltage regulator enable" ]
    pub fn usbregen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Sopt1cfg {
    register: ::volatile_register::RW<u32>,
}

impl Sopt1cfg {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Sopt1cfgR, &'w mut Sopt1cfgW) -> &'w mut Sopt1cfgW
    {
        let bits = self.register.read();
        let r = Sopt1cfgR { bits: bits };
        let mut w = Sopt1cfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Sopt1cfgR {
        Sopt1cfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Sopt1cfgW) -> &mut Sopt1cfgW
    {
        let mut w = Sopt1cfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sopt1cfgR {
    bits: u32,
}

impl Sopt1cfgR {
    # [ doc = "Bit 24 - USB voltage regulator enable write enable" ]
    pub fn urwe(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - USB voltage regulator VLP standby write enable" ]
    pub fn uvswe(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - USB voltage regulator stop standby write enable" ]
    pub fn usswe(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sopt1cfgW {
    bits: u32,
}

impl Sopt1cfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Sopt1cfgW { bits: 0 }
    }
    # [ doc = "Bit 24 - USB voltage regulator enable write enable" ]
    pub fn urwe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - USB voltage regulator VLP standby write enable" ]
    pub fn uvswe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - USB voltage regulator stop standby write enable" ]
    pub fn usswe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Sopt2 {
    register: ::volatile_register::RW<u32>,
}

impl Sopt2 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Sopt2R, &'w mut Sopt2W) -> &'w mut Sopt2W
    {
        let bits = self.register.read();
        let r = Sopt2R { bits: bits };
        let mut w = Sopt2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Sopt2R {
        Sopt2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Sopt2W) -> &mut Sopt2W
    {
        let mut w = Sopt2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sopt2R {
    bits: u32,
}

impl Sopt2R {
    # [ doc = "Bit 4 - RTC clock out select" ]
    pub fn rtcclkoutsel(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 5:7 - CLKOUT select" ]
    pub fn clkoutsel(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 5u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 11 - PTD7 pad drive strength" ]
    pub fn ptd7pad(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Debug trace clock select" ]
    pub fn traceclksel(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - PLL/FLL clock select" ]
    pub fn pllfllsel(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - USB clock source select" ]
    pub fn usbsrc(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sopt2W {
    bits: u32,
}

impl Sopt2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Sopt2W { bits: 4096 }
    }
    # [ doc = "Bit 4 - RTC clock out select" ]
    pub fn rtcclkoutsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 5:7 - CLKOUT select" ]
    pub fn clkoutsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 5u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - PTD7 pad drive strength" ]
    pub fn ptd7pad(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Debug trace clock select" ]
    pub fn traceclksel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - PLL/FLL clock select" ]
    pub fn pllfllsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - USB clock source select" ]
    pub fn usbsrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Sopt4 {
    register: ::volatile_register::RW<u32>,
}

impl Sopt4 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Sopt4R, &'w mut Sopt4W) -> &'w mut Sopt4W
    {
        let bits = self.register.read();
        let r = Sopt4R { bits: bits };
        let mut w = Sopt4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Sopt4R {
        Sopt4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Sopt4W) -> &mut Sopt4W
    {
        let mut w = Sopt4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sopt4R {
    bits: u32,
}

impl Sopt4R {
    # [ doc = "Bit 0 - FTM0 Fault 0 Select" ]
    pub fn ftm0flt0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - FTM0 Fault 1 Select" ]
    pub fn ftm0flt1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - FTM1 Fault 0 Select" ]
    pub fn ftm1flt0(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - FTM1 channel 0 input capture source select" ]
    pub fn ftm1ch0src(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 24 - FlexTimer 0 External Clock Pin Select" ]
    pub fn ftm0clksel(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - FTM1 External Clock Pin Select" ]
    pub fn ftm1clksel(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 28 - FlexTimer 0 Hardware Trigger 0 Source Select" ]
    pub fn ftm0trg0src(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sopt4W {
    bits: u32,
}

impl Sopt4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Sopt4W { bits: 0 }
    }
    # [ doc = "Bit 0 - FTM0 Fault 0 Select" ]
    pub fn ftm0flt0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - FTM0 Fault 1 Select" ]
    pub fn ftm0flt1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - FTM1 Fault 0 Select" ]
    pub fn ftm1flt0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - FTM1 channel 0 input capture source select" ]
    pub fn ftm1ch0src(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 24 - FlexTimer 0 External Clock Pin Select" ]
    pub fn ftm0clksel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - FTM1 External Clock Pin Select" ]
    pub fn ftm1clksel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - FlexTimer 0 Hardware Trigger 0 Source Select" ]
    pub fn ftm0trg0src(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Sopt5 {
    register: ::volatile_register::RW<u32>,
}

impl Sopt5 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Sopt5R, &'w mut Sopt5W) -> &'w mut Sopt5W
    {
        let bits = self.register.read();
        let r = Sopt5R { bits: bits };
        let mut w = Sopt5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Sopt5R {
        Sopt5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Sopt5W) -> &mut Sopt5W
    {
        let mut w = Sopt5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sopt5R {
    bits: u32,
}

impl Sopt5R {
    # [ doc = "Bit 0 - UART 0 transmit data source select" ]
    pub fn uart0txsrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:3 - UART 0 receive data source select" ]
    pub fn uart0rxsrc(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 4 - UART 1 transmit data source select" ]
    pub fn uart1txsrc(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 6:7 - UART 1 receive data source select" ]
    pub fn uart1rxsrc(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sopt5W {
    bits: u32,
}

impl Sopt5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Sopt5W { bits: 0 }
    }
    # [ doc = "Bit 0 - UART 0 transmit data source select" ]
    pub fn uart0txsrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:3 - UART 0 receive data source select" ]
    pub fn uart0rxsrc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 4 - UART 1 transmit data source select" ]
    pub fn uart1txsrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 6:7 - UART 1 receive data source select" ]
    pub fn uart1rxsrc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Sopt7 {
    register: ::volatile_register::RW<u32>,
}

impl Sopt7 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Sopt7R, &'w mut Sopt7W) -> &'w mut Sopt7W
    {
        let bits = self.register.read();
        let r = Sopt7R { bits: bits };
        let mut w = Sopt7W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Sopt7R {
        Sopt7R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Sopt7W) -> &mut Sopt7W
    {
        let mut w = Sopt7W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sopt7R {
    bits: u32,
}

impl Sopt7R {
    # [ doc = "Bits 0:3 - ADC0 trigger select" ]
    pub fn adc0trgsel(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 4 - ADC0 pretrigger select" ]
    pub fn adc0pretrgsel(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - ADC0 alternate trigger enable" ]
    pub fn adc0alttrgen(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sopt7W {
    bits: u32,
}

impl Sopt7W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Sopt7W { bits: 0 }
    }
    # [ doc = "Bits 0:3 - ADC0 trigger select" ]
    pub fn adc0trgsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 4 - ADC0 pretrigger select" ]
    pub fn adc0pretrgsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - ADC0 alternate trigger enable" ]
    pub fn adc0alttrgen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Sdid {
    register: ::volatile_register::RO<u32>,
}

impl Sdid {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> SdidR {
        SdidR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SdidR {
    bits: u32,
}

impl SdidR {
    # [ doc = "Bits 0:3 - Pincount identification" ]
    pub fn pinid(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:6 - Kinetis family identification" ]
    pub fn famid(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:15 - Device revision number" ]
    pub fn revid(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ repr ( C ) ]
pub struct Scgc4 {
    register: ::volatile_register::RW<u32>,
}

impl Scgc4 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Scgc4R, &'w mut Scgc4W) -> &'w mut Scgc4W
    {
        let bits = self.register.read();
        let r = Scgc4R { bits: bits };
        let mut w = Scgc4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Scgc4R {
        Scgc4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Scgc4W) -> &mut Scgc4W
    {
        let mut w = Scgc4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Scgc4R {
    bits: u32,
}

impl Scgc4R {
    # [ doc = "Bit 1 - EWM Clock Gate Control" ]
    pub fn ewm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - CMT Clock Gate Control" ]
    pub fn cmt(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - I2C0 Clock Gate Control" ]
    pub fn i2c0(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - UART0 Clock Gate Control" ]
    pub fn uart0(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - UART1 Clock Gate Control" ]
    pub fn uart1(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - UART2 Clock Gate Control" ]
    pub fn uart2(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - USB Clock Gate Control" ]
    pub fn usbotg(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Comparator Clock Gate Control" ]
    pub fn cmp(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - VREF Clock Gate Control" ]
    pub fn vref(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Scgc4W {
    bits: u32,
}

impl Scgc4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Scgc4W { bits: 4027580464 }
    }
    # [ doc = "Bit 1 - EWM Clock Gate Control" ]
    pub fn ewm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - CMT Clock Gate Control" ]
    pub fn cmt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - I2C0 Clock Gate Control" ]
    pub fn i2c0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - UART0 Clock Gate Control" ]
    pub fn uart0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - UART1 Clock Gate Control" ]
    pub fn uart1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - UART2 Clock Gate Control" ]
    pub fn uart2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - USB Clock Gate Control" ]
    pub fn usbotg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Comparator Clock Gate Control" ]
    pub fn cmp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - VREF Clock Gate Control" ]
    pub fn vref(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Scgc5 {
    register: ::volatile_register::RW<u32>,
}

impl Scgc5 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Scgc5R, &'w mut Scgc5W) -> &'w mut Scgc5W
    {
        let bits = self.register.read();
        let r = Scgc5R { bits: bits };
        let mut w = Scgc5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Scgc5R {
        Scgc5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Scgc5W) -> &mut Scgc5W
    {
        let mut w = Scgc5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Scgc5R {
    bits: u32,
}

impl Scgc5R {
    # [ doc = "Bit 0 - Low Power Timer Access Control" ]
    pub fn lptimer(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - TSI Clock Gate Control" ]
    pub fn tsi(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Port A Clock Gate Control" ]
    pub fn porta(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Port B Clock Gate Control" ]
    pub fn portb(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Port C Clock Gate Control" ]
    pub fn portc(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Port D Clock Gate Control" ]
    pub fn portd(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Port E Clock Gate Control" ]
    pub fn porte(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Scgc5W {
    bits: u32,
}

impl Scgc5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Scgc5W { bits: 262530 }
    }
    # [ doc = "Bit 0 - Low Power Timer Access Control" ]
    pub fn lptimer(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - TSI Clock Gate Control" ]
    pub fn tsi(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Port A Clock Gate Control" ]
    pub fn porta(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Port B Clock Gate Control" ]
    pub fn portb(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Port C Clock Gate Control" ]
    pub fn portc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Port D Clock Gate Control" ]
    pub fn portd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Port E Clock Gate Control" ]
    pub fn porte(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Scgc6 {
    register: ::volatile_register::RW<u32>,
}

impl Scgc6 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Scgc6R, &'w mut Scgc6W) -> &'w mut Scgc6W
    {
        let bits = self.register.read();
        let r = Scgc6R { bits: bits };
        let mut w = Scgc6W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Scgc6R {
        Scgc6R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Scgc6W) -> &mut Scgc6W
    {
        let mut w = Scgc6W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Scgc6R {
    bits: u32,
}

impl Scgc6R {
    # [ doc = "Bit 0 - Flash Memory Clock Gate Control" ]
    pub fn ftfl(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - DMA Mux Clock Gate Control" ]
    pub fn dmamux(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - SPI0 Clock Gate Control" ]
    pub fn spi0(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - I2S Clock Gate Control" ]
    pub fn i2s(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - CRC Clock Gate Control" ]
    pub fn crc(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - USB DCD Clock Gate Control" ]
    pub fn usbdcd(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - PDB Clock Gate Control" ]
    pub fn pdb(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - PIT Clock Gate Control" ]
    pub fn pit(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - FTM0 Clock Gate Control" ]
    pub fn ftm0(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - FTM1 Clock Gate Control" ]
    pub fn ftm1(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 27 - ADC0 Clock Gate Control" ]
    pub fn adc0(&self) -> bool {
        const OFFSET: u8 = 27u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - RTC Access Control" ]
    pub fn rtc(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Scgc6W {
    bits: u32,
}

impl Scgc6W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Scgc6W { bits: 1073741825 }
    }
    # [ doc = "Bit 0 - Flash Memory Clock Gate Control" ]
    pub fn ftfl(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - DMA Mux Clock Gate Control" ]
    pub fn dmamux(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - SPI0 Clock Gate Control" ]
    pub fn spi0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - I2S Clock Gate Control" ]
    pub fn i2s(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - CRC Clock Gate Control" ]
    pub fn crc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - USB DCD Clock Gate Control" ]
    pub fn usbdcd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - PDB Clock Gate Control" ]
    pub fn pdb(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - PIT Clock Gate Control" ]
    pub fn pit(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - FTM0 Clock Gate Control" ]
    pub fn ftm0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - FTM1 Clock Gate Control" ]
    pub fn ftm1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - ADC0 Clock Gate Control" ]
    pub fn adc0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - RTC Access Control" ]
    pub fn rtc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Scgc7 {
    register: ::volatile_register::RW<u32>,
}

impl Scgc7 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Scgc7R, &'w mut Scgc7W) -> &'w mut Scgc7W
    {
        let bits = self.register.read();
        let r = Scgc7R { bits: bits };
        let mut w = Scgc7W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Scgc7R {
        Scgc7R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Scgc7W) -> &mut Scgc7W
    {
        let mut w = Scgc7W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Scgc7R {
    bits: u32,
}

impl Scgc7R {
    # [ doc = "Bit 1 - DMA Clock Gate Control" ]
    pub fn dma(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Scgc7W {
    bits: u32,
}

impl Scgc7W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Scgc7W { bits: 2 }
    }
    # [ doc = "Bit 1 - DMA Clock Gate Control" ]
    pub fn dma(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Clkdiv1 {
    register: ::volatile_register::RW<u32>,
}

impl Clkdiv1 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Clkdiv1R, &'w mut Clkdiv1W) -> &'w mut Clkdiv1W
    {
        let bits = self.register.read();
        let r = Clkdiv1R { bits: bits };
        let mut w = Clkdiv1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Clkdiv1R {
        Clkdiv1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Clkdiv1W) -> &mut Clkdiv1W
    {
        let mut w = Clkdiv1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Clkdiv1R {
    bits: u32,
}

impl Clkdiv1R {
    # [ doc = "Bits 16:19 - Clock 4 output divider value" ]
    pub fn outdiv4(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - Clock 2 output divider value" ]
    pub fn outdiv2(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 28:31 - Clock 1 output divider value" ]
    pub fn outdiv1(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Clkdiv1W {
    bits: u32,
}

impl Clkdiv1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Clkdiv1W { bits: 0 }
    }
    # [ doc = "Bits 16:19 - Clock 4 output divider value" ]
    pub fn outdiv4(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - Clock 2 output divider value" ]
    pub fn outdiv2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 28:31 - Clock 1 output divider value" ]
    pub fn outdiv1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 28u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Clkdiv2 {
    register: ::volatile_register::RW<u32>,
}

impl Clkdiv2 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Clkdiv2R, &'w mut Clkdiv2W) -> &'w mut Clkdiv2W
    {
        let bits = self.register.read();
        let r = Clkdiv2R { bits: bits };
        let mut w = Clkdiv2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Clkdiv2R {
        Clkdiv2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Clkdiv2W) -> &mut Clkdiv2W
    {
        let mut w = Clkdiv2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Clkdiv2R {
    bits: u32,
}

impl Clkdiv2R {
    # [ doc = "Bit 0 - USB clock divider fraction" ]
    pub fn usbfrac(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:3 - USB clock divider divisor" ]
    pub fn usbdiv(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Clkdiv2W {
    bits: u32,
}

impl Clkdiv2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Clkdiv2W { bits: 0 }
    }
    # [ doc = "Bit 0 - USB clock divider fraction" ]
    pub fn usbfrac(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:3 - USB clock divider divisor" ]
    pub fn usbdiv(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Fcfg1 {
    register: ::volatile_register::RW<u32>,
}

impl Fcfg1 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Fcfg1R, &'w mut Fcfg1W) -> &'w mut Fcfg1W
    {
        let bits = self.register.read();
        let r = Fcfg1R { bits: bits };
        let mut w = Fcfg1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Fcfg1R {
        Fcfg1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Fcfg1W) -> &mut Fcfg1W
    {
        let mut w = Fcfg1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Fcfg1R {
    bits: u32,
}

impl Fcfg1R {
    # [ doc = "Bit 0 - Flash Disable" ]
    pub fn flashdis(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Flash Doze" ]
    pub fn flashdoze(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:11 - FlexNVM partition" ]
    pub fn depart(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - EEPROM size" ]
    pub fn eesize(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - Program flash size" ]
    pub fn pfsize(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 28:31 - FlexNVM size" ]
    pub fn nvmsize(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Fcfg1W {
    bits: u32,
}

impl Fcfg1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Fcfg1W { bits: 0 }
    }
    # [ doc = "Bit 0 - Flash Disable" ]
    pub fn flashdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Flash Doze" ]
    pub fn flashdoze(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Fcfg2 {
    register: ::volatile_register::RO<u32>,
}

impl Fcfg2 {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> Fcfg2R {
        Fcfg2R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Fcfg2R {
    bits: u32,
}

impl Fcfg2R {
    # [ doc = "Bits 16:22 - Max address block 1" ]
    pub fn maxaddr1(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 23 - Program flash" ]
    pub fn pflsh(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 24:30 - Max address block 0" ]
    pub fn maxaddr0(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ repr ( C ) ]
pub struct Uidh {
    register: ::volatile_register::RO<u32>,
}

impl Uidh {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> UidhR {
        UidhR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct UidhR {
    bits: u32,
}

impl UidhR {
    # [ doc = "Bits 0:31 - Unique Identification" ]
    pub fn uid(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ repr ( C ) ]
pub struct Uidmh {
    register: ::volatile_register::RO<u32>,
}

impl Uidmh {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> UidmhR {
        UidmhR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct UidmhR {
    bits: u32,
}

impl UidmhR {
    # [ doc = "Bits 0:31 - Unique Identification" ]
    pub fn uid(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ repr ( C ) ]
pub struct Uidml {
    register: ::volatile_register::RO<u32>,
}

impl Uidml {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> UidmlR {
        UidmlR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct UidmlR {
    bits: u32,
}

impl UidmlR {
    # [ doc = "Bits 0:31 - Unique Identification" ]
    pub fn uid(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ repr ( C ) ]
pub struct Uidl {
    register: ::volatile_register::RO<u32>,
}

impl Uidl {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> UidlR {
        UidlR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct UidlR {
    bits: u32,
}

impl UidlR {
    # [ doc = "Bits 0:31 - Unique Identification" ]
    pub fn uid(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}
