# [ doc = "Pin Control and Interrupts" ]
# [ repr ( C ) ]
pub struct Port {
    # [ doc = "0x00 - Pin Control Register n" ]
    pub pcr0: Pcr,
    # [ doc = "0x04 - Pin Control Register n" ]
    pub pcr1: Pcr,
    # [ doc = "0x08 - Pin Control Register n" ]
    pub pcr2: Pcr,
    # [ doc = "0x0c - Pin Control Register n" ]
    pub pcr3: Pcr,
    # [ doc = "0x10 - Pin Control Register n" ]
    pub pcr4: Pcr,
    # [ doc = "0x14 - Pin Control Register n" ]
    pub pcr5: Pcr,
    # [ doc = "0x18 - Pin Control Register n" ]
    pub pcr6: Pcr,
    # [ doc = "0x1c - Pin Control Register n" ]
    pub pcr7: Pcr,
    # [ doc = "0x20 - Pin Control Register n" ]
    pub pcr8: Pcr,
    # [ doc = "0x24 - Pin Control Register n" ]
    pub pcr9: Pcr,
    # [ doc = "0x28 - Pin Control Register n" ]
    pub pcr10: Pcr,
    # [ doc = "0x2c - Pin Control Register n" ]
    pub pcr11: Pcr,
    # [ doc = "0x30 - Pin Control Register n" ]
    pub pcr12: Pcr,
    # [ doc = "0x34 - Pin Control Register n" ]
    pub pcr13: Pcr,
    # [ doc = "0x38 - Pin Control Register n" ]
    pub pcr14: Pcr,
    # [ doc = "0x3c - Pin Control Register n" ]
    pub pcr15: Pcr,
    # [ doc = "0x40 - Pin Control Register n" ]
    pub pcr16: Pcr,
    # [ doc = "0x44 - Pin Control Register n" ]
    pub pcr17: Pcr,
    # [ doc = "0x48 - Pin Control Register n" ]
    pub pcr18: Pcr,
    # [ doc = "0x4c - Pin Control Register n" ]
    pub pcr19: Pcr,
    # [ doc = "0x50 - Pin Control Register n" ]
    pub pcr20: Pcr,
    # [ doc = "0x54 - Pin Control Register n" ]
    pub pcr21: Pcr,
    # [ doc = "0x58 - Pin Control Register n" ]
    pub pcr22: Pcr,
    # [ doc = "0x5c - Pin Control Register n" ]
    pub pcr23: Pcr,
    # [ doc = "0x60 - Pin Control Register n" ]
    pub pcr24: Pcr,
    # [ doc = "0x64 - Pin Control Register n" ]
    pub pcr25: Pcr,
    # [ doc = "0x68 - Pin Control Register n" ]
    pub pcr26: Pcr,
    # [ doc = "0x6c - Pin Control Register n" ]
    pub pcr27: Pcr,
    # [ doc = "0x70 - Pin Control Register n" ]
    pub pcr28: Pcr,
    # [ doc = "0x74 - Pin Control Register n" ]
    pub pcr29: Pcr,
    # [ doc = "0x78 - Pin Control Register n" ]
    pub pcr30: Pcr,
    # [ doc = "0x7c - Pin Control Register n" ]
    pub pcr31: Pcr,
    # [ doc = "0x80 - Global Pin Control Low Register" ]
    pub gpclr: Gpclr,
    # [ doc = "0x84 - Global Pin Control High Register" ]
    pub gpchr: Gpchr,
    _reserved0: [u8; 24usize],
    # [ doc = "0xa0 - Interrupt Status Flag Register" ]
    pub isfr: Isfr,
}

# [ repr ( C ) ]
pub struct Pcr {
    register: ::volatile_register::RW<u32>,
}

impl Pcr {
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
        where for<'w> F: FnOnce(&PcrR, &'w mut PcrW) -> &'w mut PcrW
    {
        let bits = self.register.read();
        let r = PcrR { bits: bits };
        let mut w = PcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PcrR {
        PcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PcrW) -> &mut PcrW
    {
        let mut w = PcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PcrR {
    bits: u32,
}

impl PcrR {
    # [ doc = "Bit 0 - Pull Select" ]
    pub fn ps(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Pull Enable" ]
    pub fn pe(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Slew Rate Enable" ]
    pub fn sre(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Passive Filter Enable" ]
    pub fn pfe(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Open Drain Enable" ]
    pub fn ode(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Drive Strength Enable" ]
    pub fn dse(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:10 - Pin Mux Control" ]
    pub fn mux(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Lock Register" ]
    pub fn lk(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 16:19 - Interrupt Configuration" ]
    pub fn irqc(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 24 - Interrupt Status Flag" ]
    pub fn isf(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PcrW {
    bits: u32,
}

impl PcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PcrW { bits: 0 }
    }
    # [ doc = "Bit 0 - Pull Select" ]
    pub fn ps(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Pull Enable" ]
    pub fn pe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Slew Rate Enable" ]
    pub fn sre(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Passive Filter Enable" ]
    pub fn pfe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Open Drain Enable" ]
    pub fn ode(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Drive Strength Enable" ]
    pub fn dse(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 8:10 - Pin Mux Control" ]
    pub fn mux(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Lock Register" ]
    pub fn lk(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 16:19 - Interrupt Configuration" ]
    pub fn irqc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 24 - Interrupt Status Flag" ]
    pub fn isf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Gpclr {
    register: ::volatile_register::WO<u32>,
}

impl Gpclr {
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut GpclrW) -> &mut GpclrW
    {
        let mut w = GpclrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct GpclrW {
    bits: u32,
}

impl GpclrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        GpclrW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Global Pin Write Data" ]
    pub fn gpwd(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - Global Pin Write Enable" ]
    pub fn gpwe(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Gpchr {
    register: ::volatile_register::WO<u32>,
}

impl Gpchr {
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut GpchrW) -> &mut GpchrW
    {
        let mut w = GpchrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct GpchrW {
    bits: u32,
}

impl GpchrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        GpchrW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Global Pin Write Data" ]
    pub fn gpwd(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - Global Pin Write Enable" ]
    pub fn gpwe(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Isfr {
    register: ::volatile_register::RW<u32>,
}

impl Isfr {
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
        where for<'w> F: FnOnce(&IsfrR, &'w mut IsfrW) -> &'w mut IsfrW
    {
        let bits = self.register.read();
        let r = IsfrR { bits: bits };
        let mut w = IsfrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> IsfrR {
        IsfrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut IsfrW) -> &mut IsfrW
    {
        let mut w = IsfrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IsfrR {
    bits: u32,
}

impl IsfrR {
    # [ doc = "Bits 0:31 - Interrupt Status Flag" ]
    pub fn isf(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IsfrW {
    bits: u32,
}

impl IsfrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IsfrW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - Interrupt Status Flag" ]
    pub fn isf(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
