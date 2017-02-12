# [ doc = "Serial Communication Interface" ]
# [ repr ( C ) ]
pub struct Uart {
    # [ doc = "0x00 - UART Baud Rate Registers:High" ]
    pub bdh: Bdh,
    # [ doc = "0x01 - UART Baud Rate Registers: Low" ]
    pub bdl: Bdl,
    # [ doc = "0x02 - UART Control Register 1" ]
    pub c1: C1,
    # [ doc = "0x03 - UART Control Register 2" ]
    pub c2: C2,
    # [ doc = "0x04 - UART Status Register 1" ]
    pub s1: S1,
    # [ doc = "0x05 - UART Status Register 2" ]
    pub s2: S2,
    # [ doc = "0x06 - UART Control Register 3" ]
    pub c3: C3,
    # [ doc = "0x07 - UART Data Register" ]
    pub d: D,
    # [ doc = "0x08 - UART Match Address Registers 1" ]
    pub ma1: Ma1,
    # [ doc = "0x09 - UART Match Address Registers 2" ]
    pub ma2: Ma2,
    # [ doc = "0x0a - UART Control Register 4" ]
    pub c4: C4,
    # [ doc = "0x0b - UART Control Register 5" ]
    pub c5: C5,
    # [ doc = "0x0c - UART Extended Data Register" ]
    pub ed: Ed,
    # [ doc = "0x0d - UART Modem Register" ]
    pub modem: Modem,
    # [ doc = "0x0e - UART Infrared Register" ]
    pub ir: Ir,
    _reserved0: [u8; 1usize],
    # [ doc = "0x10 - UART FIFO Parameters" ]
    pub pfifo: Pfifo,
    # [ doc = "0x11 - UART FIFO Control Register" ]
    pub cfifo: Cfifo,
    # [ doc = "0x12 - UART FIFO Status Register" ]
    pub sfifo: Sfifo,
    # [ doc = "0x13 - UART FIFO Transmit Watermark" ]
    pub twfifo: Twfifo,
    # [ doc = "0x14 - UART FIFO Transmit Count" ]
    pub tcfifo: Tcfifo,
    # [ doc = "0x15 - UART FIFO Receive Watermark" ]
    pub rwfifo: Rwfifo,
    # [ doc = "0x16 - UART FIFO Receive Count" ]
    pub rcfifo: Rcfifo,
}

# [ repr ( C ) ]
pub struct Bdh {
    register: ::volatile_register::RW<u8>,
}

impl Bdh {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u8)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u8) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BdhR, &'w mut BdhW) -> &'w mut BdhW
    {
        let bits = self.register.read();
        let r = BdhR { bits: bits };
        let mut w = BdhW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BdhR {
        BdhR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BdhW) -> &mut BdhW
    {
        let mut w = BdhW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BdhR {
    bits: u8,
}

impl BdhR {
    # [ doc = "Bits 0:4 - UART Baud Rate Bits" ]
    pub fn sbr(&self) -> u8 {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 6 - RxD Input Active Edge Interrupt Enable" ]
    pub fn rxedgie(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - LIN Break Detect Interrupt Enable" ]
    pub fn lbkdie(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BdhW {
    bits: u8,
}

impl BdhW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BdhW { bits: 0 }
    }
    # [ doc = "Bits 0:4 - UART Baud Rate Bits" ]
    pub fn sbr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
    # [ doc = "Bit 6 - RxD Input Active Edge Interrupt Enable" ]
    pub fn rxedgie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - LIN Break Detect Interrupt Enable" ]
    pub fn lbkdie(&mut self, value: bool) -> &mut Self {
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
pub struct Bdl {
    register: ::volatile_register::RW<u8>,
}

impl Bdl {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u8)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u8) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BdlR, &'w mut BdlW) -> &'w mut BdlW
    {
        let bits = self.register.read();
        let r = BdlR { bits: bits };
        let mut w = BdlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BdlR {
        BdlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BdlW) -> &mut BdlW
    {
        let mut w = BdlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BdlR {
    bits: u8,
}

impl BdlR {
    # [ doc = "Bits 0:7 - UART Baud Rate Bits" ]
    pub fn sbr(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BdlW {
    bits: u8,
}

impl BdlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BdlW { bits: 4 }
    }
    # [ doc = "Bits 0:7 - UART Baud Rate Bits" ]
    pub fn sbr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct C1 {
    register: ::volatile_register::RW<u8>,
}

impl C1 {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u8)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u8) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&C1R, &'w mut C1W) -> &'w mut C1W
    {
        let bits = self.register.read();
        let r = C1R { bits: bits };
        let mut w = C1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> C1R {
        C1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut C1W) -> &mut C1W
    {
        let mut w = C1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct C1R {
    bits: u8,
}

impl C1R {
    # [ doc = "Bit 0 - Parity Type" ]
    pub fn pt(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Parity Enable" ]
    pub fn pe(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Idle Line Type Select" ]
    pub fn ilt(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Receiver Wakeup Method Select" ]
    pub fn wake(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - 9-bit or 8-bit Mode Select" ]
    pub fn m(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Receiver Source Select" ]
    pub fn rsrc(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - UART Stops in Wait Mode" ]
    pub fn uartswai(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Loop Mode Select" ]
    pub fn loops(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct C1W {
    bits: u8,
}

impl C1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        C1W { bits: 0 }
    }
    # [ doc = "Bit 0 - Parity Type" ]
    pub fn pt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Parity Enable" ]
    pub fn pe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Idle Line Type Select" ]
    pub fn ilt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Receiver Wakeup Method Select" ]
    pub fn wake(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - 9-bit or 8-bit Mode Select" ]
    pub fn m(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Receiver Source Select" ]
    pub fn rsrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - UART Stops in Wait Mode" ]
    pub fn uartswai(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Loop Mode Select" ]
    pub fn loops(&mut self, value: bool) -> &mut Self {
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
pub struct C2 {
    register: ::volatile_register::RW<u8>,
}

impl C2 {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u8)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u8) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&C2R, &'w mut C2W) -> &'w mut C2W
    {
        let bits = self.register.read();
        let r = C2R { bits: bits };
        let mut w = C2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> C2R {
        C2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut C2W) -> &mut C2W
    {
        let mut w = C2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct C2R {
    bits: u8,
}

impl C2R {
    # [ doc = "Bit 0 - Send Break" ]
    pub fn sbk(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Receiver Wakeup Control" ]
    pub fn rwu(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Receiver Enable" ]
    pub fn re(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Transmitter Enable" ]
    pub fn te(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Idle Line Interrupt Enable" ]
    pub fn ilie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Receiver Full Interrupt or DMA Transfer Enable" ]
    pub fn rie(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Transmission Complete Interrupt Enable" ]
    pub fn tcie(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transmitter Interrupt or DMA Transfer Enable." ]
    pub fn tie(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct C2W {
    bits: u8,
}

impl C2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        C2W { bits: 0 }
    }
    # [ doc = "Bit 0 - Send Break" ]
    pub fn sbk(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Receiver Wakeup Control" ]
    pub fn rwu(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Receiver Enable" ]
    pub fn re(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Transmitter Enable" ]
    pub fn te(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Idle Line Interrupt Enable" ]
    pub fn ilie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Receiver Full Interrupt or DMA Transfer Enable" ]
    pub fn rie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Transmission Complete Interrupt Enable" ]
    pub fn tcie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transmitter Interrupt or DMA Transfer Enable." ]
    pub fn tie(&mut self, value: bool) -> &mut Self {
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
pub struct S1 {
    register: ::volatile_register::RO<u8>,
}

impl S1 {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub fn read(&self) -> S1R {
        S1R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S1R {
    bits: u8,
}

impl S1R {
    # [ doc = "Bit 0 - Parity Error Flag" ]
    pub fn pf(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Framing Error Flag" ]
    pub fn fe(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Noise Flag" ]
    pub fn nf(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Receiver Overrun Flag" ]
    pub fn or(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Idle Line Flag" ]
    pub fn idle(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Receive Data Register Full Flag" ]
    pub fn rdrf(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Transmit Complete Flag" ]
    pub fn tc(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transmit Data Register Empty Flag" ]
    pub fn tdre(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ repr ( C ) ]
pub struct S2 {
    register: ::volatile_register::RW<u8>,
}

impl S2 {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u8)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u8) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S2R, &'w mut S2W) -> &'w mut S2W
    {
        let bits = self.register.read();
        let r = S2R { bits: bits };
        let mut w = S2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S2R {
        S2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S2W) -> &mut S2W
    {
        let mut w = S2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S2R {
    bits: u8,
}

impl S2R {
    # [ doc = "Bit 0 - Receiver Active Flag" ]
    pub fn raf(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - LIN Break Detection Enable" ]
    pub fn lbkde(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Break Transmit Character Length" ]
    pub fn brk13(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Receive Wakeup Idle Detect" ]
    pub fn rwuid(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Receive Data Inversion" ]
    pub fn rxinv(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Most Significant Bit First" ]
    pub fn msbf(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - RxD Pin Active Edge Interrupt Flag" ]
    pub fn rxedgif(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - LIN Break Detect Interrupt Flag" ]
    pub fn lbkdif(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S2W {
    bits: u8,
}

impl S2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S2W { bits: 0 }
    }
    # [ doc = "Bit 1 - LIN Break Detection Enable" ]
    pub fn lbkde(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Break Transmit Character Length" ]
    pub fn brk13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Receive Wakeup Idle Detect" ]
    pub fn rwuid(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Receive Data Inversion" ]
    pub fn rxinv(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Most Significant Bit First" ]
    pub fn msbf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - RxD Pin Active Edge Interrupt Flag" ]
    pub fn rxedgif(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - LIN Break Detect Interrupt Flag" ]
    pub fn lbkdif(&mut self, value: bool) -> &mut Self {
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
pub struct C3 {
    register: ::volatile_register::RW<u8>,
}

impl C3 {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u8)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u8) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&C3R, &'w mut C3W) -> &'w mut C3W
    {
        let bits = self.register.read();
        let r = C3R { bits: bits };
        let mut w = C3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> C3R {
        C3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut C3W) -> &mut C3W
    {
        let mut w = C3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct C3R {
    bits: u8,
}

impl C3R {
    # [ doc = "Bit 0 - Parity Error Interrupt Enable" ]
    pub fn peie(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Framing Error Interrupt Enable" ]
    pub fn feie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Noise Error Interrupt Enable" ]
    pub fn neie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Overrun Error Interrupt Enable" ]
    pub fn orie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Transmit Data Inversion." ]
    pub fn txinv(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Transmitter Pin Data Direction in Single-Wire mode" ]
    pub fn txdir(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Transmit Bit 8" ]
    pub fn t8(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Received Bit 8" ]
    pub fn r8(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct C3W {
    bits: u8,
}

impl C3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        C3W { bits: 0 }
    }
    # [ doc = "Bit 0 - Parity Error Interrupt Enable" ]
    pub fn peie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Framing Error Interrupt Enable" ]
    pub fn feie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Noise Error Interrupt Enable" ]
    pub fn neie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Overrun Error Interrupt Enable" ]
    pub fn orie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Transmit Data Inversion." ]
    pub fn txinv(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Transmitter Pin Data Direction in Single-Wire mode" ]
    pub fn txdir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Transmit Bit 8" ]
    pub fn t8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct D {
    register: ::volatile_register::RW<u8>,
}

impl D {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u8)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u8) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DR, &'w mut DW) -> &'w mut DW
    {
        let bits = self.register.read();
        let r = DR { bits: bits };
        let mut w = DW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DR {
        DR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DW) -> &mut DW
    {
        let mut w = DW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DR {
    bits: u8,
}

impl DR {
    # [ doc = "Bits 0:7 - Reads return the contents of the read-only receive data register and writes go to the write-only transmit data register" ]
    pub fn rt(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DW {
    bits: u8,
}

impl DW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DW { bits: 0 }
    }
    # [ doc = "Bits 0:7 - Reads return the contents of the read-only receive data register and writes go to the write-only transmit data register" ]
    pub fn rt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ma1 {
    register: ::volatile_register::RW<u8>,
}

impl Ma1 {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u8)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u8) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ma1R, &'w mut Ma1W) -> &'w mut Ma1W
    {
        let bits = self.register.read();
        let r = Ma1R { bits: bits };
        let mut w = Ma1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ma1R {
        Ma1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ma1W) -> &mut Ma1W
    {
        let mut w = Ma1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ma1R {
    bits: u8,
}

impl Ma1R {
    # [ doc = "Bits 0:7 - Match Address" ]
    pub fn ma(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ma1W {
    bits: u8,
}

impl Ma1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ma1W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - Match Address" ]
    pub fn ma(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ma2 {
    register: ::volatile_register::RW<u8>,
}

impl Ma2 {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u8)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u8) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ma2R, &'w mut Ma2W) -> &'w mut Ma2W
    {
        let bits = self.register.read();
        let r = Ma2R { bits: bits };
        let mut w = Ma2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ma2R {
        Ma2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ma2W) -> &mut Ma2W
    {
        let mut w = Ma2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ma2R {
    bits: u8,
}

impl Ma2R {
    # [ doc = "Bits 0:7 - Match Address" ]
    pub fn ma(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ma2W {
    bits: u8,
}

impl Ma2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ma2W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - Match Address" ]
    pub fn ma(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct C4 {
    register: ::volatile_register::RW<u8>,
}

impl C4 {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u8)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u8) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&C4R, &'w mut C4W) -> &'w mut C4W
    {
        let bits = self.register.read();
        let r = C4R { bits: bits };
        let mut w = C4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> C4R {
        C4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut C4W) -> &mut C4W
    {
        let mut w = C4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct C4R {
    bits: u8,
}

impl C4R {
    # [ doc = "Bits 0:4 - Baud Rate Fine Adjust" ]
    pub fn brfa(&self) -> u8 {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 5 - 10-bit Mode select" ]
    pub fn m10(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Match Address Mode Enable 2" ]
    pub fn maen2(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Match Address Mode Enable 1" ]
    pub fn maen1(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct C4W {
    bits: u8,
}

impl C4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        C4W { bits: 0 }
    }
    # [ doc = "Bits 0:4 - Baud Rate Fine Adjust" ]
    pub fn brfa(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
    # [ doc = "Bit 5 - 10-bit Mode select" ]
    pub fn m10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Match Address Mode Enable 2" ]
    pub fn maen2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Match Address Mode Enable 1" ]
    pub fn maen1(&mut self, value: bool) -> &mut Self {
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
pub struct C5 {
    register: ::volatile_register::RW<u8>,
}

impl C5 {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u8)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u8) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&C5R, &'w mut C5W) -> &'w mut C5W
    {
        let bits = self.register.read();
        let r = C5R { bits: bits };
        let mut w = C5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> C5R {
        C5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut C5W) -> &mut C5W
    {
        let mut w = C5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct C5R {
    bits: u8,
}

impl C5R {
    # [ doc = "Bit 5 - Receiver Full DMA Select" ]
    pub fn rdmas(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transmitter DMA Select" ]
    pub fn tdmas(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct C5W {
    bits: u8,
}

impl C5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        C5W { bits: 0 }
    }
    # [ doc = "Bit 5 - Receiver Full DMA Select" ]
    pub fn rdmas(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transmitter DMA Select" ]
    pub fn tdmas(&mut self, value: bool) -> &mut Self {
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
pub struct Ed {
    register: ::volatile_register::RO<u8>,
}

impl Ed {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub fn read(&self) -> EdR {
        EdR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct EdR {
    bits: u8,
}

impl EdR {
    # [ doc = "Bit 6 - The current received dataword contained in D and C3[R8] was received with a parity error." ]
    pub fn paritye(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - The current received dataword contained in D and C3[R8] was received with noise." ]
    pub fn noisy(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ repr ( C ) ]
pub struct Modem {
    register: ::volatile_register::RW<u8>,
}

impl Modem {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u8)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u8) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ModemR, &'w mut ModemW) -> &'w mut ModemW
    {
        let bits = self.register.read();
        let r = ModemR { bits: bits };
        let mut w = ModemW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ModemR {
        ModemR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ModemW) -> &mut ModemW
    {
        let mut w = ModemW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ModemR {
    bits: u8,
}

impl ModemR {
    # [ doc = "Bit 0 - Transmitter clear-to-send enable" ]
    pub fn txctse(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Transmitter request-to-send enable" ]
    pub fn txrtse(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Transmitter request-to-send polarity" ]
    pub fn txrtspol(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Receiver request-to-send enable" ]
    pub fn rxrtse(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ModemW {
    bits: u8,
}

impl ModemW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ModemW { bits: 0 }
    }
    # [ doc = "Bit 0 - Transmitter clear-to-send enable" ]
    pub fn txctse(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Transmitter request-to-send enable" ]
    pub fn txrtse(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Transmitter request-to-send polarity" ]
    pub fn txrtspol(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Receiver request-to-send enable" ]
    pub fn rxrtse(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Ir {
    register: ::volatile_register::RW<u8>,
}

impl Ir {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u8)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u8) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&IrR, &'w mut IrW) -> &'w mut IrW
    {
        let bits = self.register.read();
        let r = IrR { bits: bits };
        let mut w = IrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> IrR {
        IrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut IrW) -> &mut IrW
    {
        let mut w = IrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IrR {
    bits: u8,
}

impl IrR {
    # [ doc = "Bits 0:1 - Transmitter narrow pulse" ]
    pub fn tnp(&self) -> u8 {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 2 - Infrared enable" ]
    pub fn iren(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IrW {
    bits: u8,
}

impl IrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IrW { bits: 0 }
    }
    # [ doc = "Bits 0:1 - Transmitter narrow pulse" ]
    pub fn tnp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
    # [ doc = "Bit 2 - Infrared enable" ]
    pub fn iren(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Pfifo {
    register: ::volatile_register::RW<u8>,
}

impl Pfifo {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u8)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u8) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PfifoR, &'w mut PfifoW) -> &'w mut PfifoW
    {
        let bits = self.register.read();
        let r = PfifoR { bits: bits };
        let mut w = PfifoW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PfifoR {
        PfifoR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PfifoW) -> &mut PfifoW
    {
        let mut w = PfifoW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PfifoR {
    bits: u8,
}

impl PfifoR {
    # [ doc = "Bits 0:2 - Receive FIFO. Buffer Depth" ]
    pub fn rxfifosize(&self) -> u8 {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 3 - Receive FIFO Enable" ]
    pub fn rxfe(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:6 - Transmit FIFO. Buffer Depth" ]
    pub fn txfifosize(&self) -> u8 {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - Transmit FIFO Enable" ]
    pub fn txfe(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PfifoW {
    bits: u8,
}

impl PfifoW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PfifoW { bits: 0 }
    }
    # [ doc = "Bit 3 - Receive FIFO Enable" ]
    pub fn rxfe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transmit FIFO Enable" ]
    pub fn txfe(&mut self, value: bool) -> &mut Self {
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
pub struct Cfifo {
    register: ::volatile_register::RW<u8>,
}

impl Cfifo {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u8)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u8) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CfifoR, &'w mut CfifoW) -> &'w mut CfifoW
    {
        let bits = self.register.read();
        let r = CfifoR { bits: bits };
        let mut w = CfifoW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CfifoR {
        CfifoR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CfifoW) -> &mut CfifoW
    {
        let mut w = CfifoW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CfifoR {
    bits: u8,
}

impl CfifoR {
    # [ doc = "Bit 0 - Receive FIFO Underflow Interrupt Enable" ]
    pub fn rxufe(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Transmit FIFO Overflow Interrupt Enable" ]
    pub fn txofe(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CfifoW {
    bits: u8,
}

impl CfifoW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CfifoW { bits: 0 }
    }
    # [ doc = "Bit 0 - Receive FIFO Underflow Interrupt Enable" ]
    pub fn rxufe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Transmit FIFO Overflow Interrupt Enable" ]
    pub fn txofe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Receive FIFO/Buffer Flush" ]
    pub fn rxflush(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transmit FIFO/Buffer Flush" ]
    pub fn txflush(&mut self, value: bool) -> &mut Self {
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
pub struct Sfifo {
    register: ::volatile_register::RW<u8>,
}

impl Sfifo {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u8)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u8) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&SfifoR, &'w mut SfifoW) -> &'w mut SfifoW
    {
        let bits = self.register.read();
        let r = SfifoR { bits: bits };
        let mut w = SfifoW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> SfifoR {
        SfifoR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut SfifoW) -> &mut SfifoW
    {
        let mut w = SfifoW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SfifoR {
    bits: u8,
}

impl SfifoR {
    # [ doc = "Bit 0 - Receiver Buffer Underflow Flag" ]
    pub fn rxuf(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Transmitter Buffer Overflow Flag" ]
    pub fn txof(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Receive Buffer/FIFO Empty" ]
    pub fn rxempt(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transmit Buffer/FIFO Empty" ]
    pub fn txempt(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SfifoW {
    bits: u8,
}

impl SfifoW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        SfifoW { bits: 192 }
    }
    # [ doc = "Bit 0 - Receiver Buffer Underflow Flag" ]
    pub fn rxuf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Transmitter Buffer Overflow Flag" ]
    pub fn txof(&mut self, value: bool) -> &mut Self {
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
pub struct Twfifo {
    register: ::volatile_register::RW<u8>,
}

impl Twfifo {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u8)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u8) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&TwfifoR, &'w mut TwfifoW) -> &'w mut TwfifoW
    {
        let bits = self.register.read();
        let r = TwfifoR { bits: bits };
        let mut w = TwfifoW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> TwfifoR {
        TwfifoR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut TwfifoW) -> &mut TwfifoW
    {
        let mut w = TwfifoW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct TwfifoR {
    bits: u8,
}

impl TwfifoR {
    # [ doc = "Bits 0:7 - Transmit Watermark" ]
    pub fn txwater(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct TwfifoW {
    bits: u8,
}

impl TwfifoW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        TwfifoW { bits: 0 }
    }
    # [ doc = "Bits 0:7 - Transmit Watermark" ]
    pub fn txwater(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Tcfifo {
    register: ::volatile_register::RO<u8>,
}

impl Tcfifo {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub fn read(&self) -> TcfifoR {
        TcfifoR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct TcfifoR {
    bits: u8,
}

impl TcfifoR {
    # [ doc = "Bits 0:7 - Transmit Counter" ]
    pub fn txcount(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ repr ( C ) ]
pub struct Rwfifo {
    register: ::volatile_register::RW<u8>,
}

impl Rwfifo {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u8)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    pub unsafe fn write_bits(&mut self, bits: u8) {
        self.register.write(bits);
    }
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&RwfifoR, &'w mut RwfifoW) -> &'w mut RwfifoW
    {
        let bits = self.register.read();
        let r = RwfifoR { bits: bits };
        let mut w = RwfifoW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> RwfifoR {
        RwfifoR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut RwfifoW) -> &mut RwfifoW
    {
        let mut w = RwfifoW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct RwfifoR {
    bits: u8,
}

impl RwfifoR {
    # [ doc = "Bits 0:7 - Receive Watermark" ]
    pub fn rxwater(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct RwfifoW {
    bits: u8,
}

impl RwfifoW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        RwfifoW { bits: 1 }
    }
    # [ doc = "Bits 0:7 - Receive Watermark" ]
    pub fn rxwater(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Rcfifo {
    register: ::volatile_register::RO<u8>,
}

impl Rcfifo {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub fn read(&self) -> RcfifoR {
        RcfifoR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct RcfifoR {
    bits: u8,
}

impl RcfifoR {
    # [ doc = "Bits 0:7 - Receive Counter" ]
    pub fn rxcount(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}
