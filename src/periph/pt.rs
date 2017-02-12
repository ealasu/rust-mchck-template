# [ doc = "General Purpose Input/Output" ]
# [ repr ( C ) ]
pub struct Pt {
    # [ doc = "0x00 - Port Data Output Register" ]
    pub pdor: Pdor,
    # [ doc = "0x04 - Port Set Output Register" ]
    pub psor: Psor,
    # [ doc = "0x08 - Port Clear Output Register" ]
    pub pcor: Pcor,
    # [ doc = "0x0c - Port Toggle Output Register" ]
    pub ptor: Ptor,
    # [ doc = "0x10 - Port Data Input Register" ]
    pub pdir: Pdir,
    # [ doc = "0x14 - Port Data Direction Register" ]
    pub pddr: Pddr,
}

# [ repr ( C ) ]
pub struct Pdor {
    register: ::volatile_register::RW<u32>,
}

impl Pdor {
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
        where for<'w> F: FnOnce(&PdorR, &'w mut PdorW) -> &'w mut PdorW
    {
        let bits = self.register.read();
        let r = PdorR { bits: bits };
        let mut w = PdorW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PdorR {
        PdorR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PdorW) -> &mut PdorW
    {
        let mut w = PdorW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PdorR {
    bits: u32,
}

impl PdorR {
    # [ doc = "Bits 0:31 - Port Data Output" ]
    pub fn pdo(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PdorW {
    bits: u32,
}

impl PdorW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PdorW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - Port Data Output" ]
    pub fn pdo(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Psor {
    register: ::volatile_register::WO<u32>,
}

impl Psor {
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut PsorW) -> &mut PsorW
    {
        let mut w = PsorW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PsorW {
    bits: u32,
}

impl PsorW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PsorW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - Port Set Output" ]
    pub fn ptso(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pcor {
    register: ::volatile_register::WO<u32>,
}

impl Pcor {
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut PcorW) -> &mut PcorW
    {
        let mut w = PcorW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PcorW {
    bits: u32,
}

impl PcorW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PcorW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - Port Clear Output" ]
    pub fn ptco(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ptor {
    register: ::volatile_register::WO<u32>,
}

impl Ptor {
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut PtorW) -> &mut PtorW
    {
        let mut w = PtorW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PtorW {
    bits: u32,
}

impl PtorW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PtorW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - Port Toggle Output" ]
    pub fn ptto(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pdir {
    register: ::volatile_register::RO<u32>,
}

impl Pdir {
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    pub fn read(&self) -> PdirR {
        PdirR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PdirR {
    bits: u32,
}

impl PdirR {
    # [ doc = "Bits 0:31 - Port Data Input" ]
    pub fn pdi(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ repr ( C ) ]
pub struct Pddr {
    register: ::volatile_register::RW<u32>,
}

impl Pddr {
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
        where for<'w> F: FnOnce(&PddrR, &'w mut PddrW) -> &'w mut PddrW
    {
        let bits = self.register.read();
        let r = PddrR { bits: bits };
        let mut w = PddrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PddrR {
        PddrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PddrW) -> &mut PddrW
    {
        let mut w = PddrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PddrR {
    bits: u32,
}

impl PddrR {
    # [ doc = "Bits 0:31 - Port data direction" ]
    pub fn pdd(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PddrW {
    bits: u32,
}

impl PddrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PddrW { bits: 0 }
    }
    # [ doc = "Bits 0:31 - Port data direction" ]
    pub fn pdd(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
