# [ doc = "Serial Communication Interface" ]
# [ repr ( C ) ]
pub struct Uart0 {
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
    _reserved1: [u8; 1usize],
    # [ doc = "0x18 - UART 7816 Control Register" ]
    pub c7816: C7816,
    # [ doc = "0x19 - UART 7816 Interrupt Enable Register" ]
    pub ie7816: Ie7816,
    # [ doc = "0x1a - UART 7816 Interrupt Status Register" ]
    pub is7816: Is7816,
    # [ doc = "0x1b - UART 7816 Wait Parameter Register" ]
    pub wp7816t0: Wp7816t0,
    # [ doc = "0x1c - UART 7816 Wait N Register" ]
    pub wn7816: Wn7816,
    # [ doc = "0x1d - UART 7816 Wait FD Register" ]
    pub wf7816: Wf7816,
    # [ doc = "0x1e - UART 7816 Error Threshold Register" ]
    pub et7816: Et7816,
    # [ doc = "0x1f - UART 7816 Transmit Length Register" ]
    pub tl7816: Tl7816,
    _reserved2: [u8; 1usize],
    # [ doc = "0x21 - UART CEA709.1-B Control Register 6" ]
    pub c6: C6,
    # [ doc = "0x22 - UART CEA709.1-B Packet Cycle Time Counter High" ]
    pub pcth: Pcth,
    # [ doc = "0x23 - UART CEA709.1-B Packet Cycle Time Counter Low" ]
    pub pctl: Pctl,
    # [ doc = "0x24 - UART CEA709.1-B Beta1 Timer" ]
    pub b1t: B1t,
    # [ doc = "0x25 - UART CEA709.1-B Secondary Delay Timer High" ]
    pub sdth: Sdth,
    # [ doc = "0x26 - UART CEA709.1-B Secondary Delay Timer Low" ]
    pub sdtl: Sdtl,
    # [ doc = "0x27 - UART CEA709.1-B Preamble" ]
    pub pre: Pre,
    # [ doc = "0x28 - UART CEA709.1-B Transmit Packet Length" ]
    pub tpl: Tpl,
    # [ doc = "0x29 - UART CEA709.1-B Interrupt Enable Register" ]
    pub ie: Ie,
    # [ doc = "0x2a - UART CEA709.1-B WBASE" ]
    pub wb: Wb,
    # [ doc = "0x2b - UART CEA709.1-B Status Register" ]
    pub s3: S3,
    # [ doc = "0x2c - UART CEA709.1-B Status Register" ]
    pub s4: S4,
    # [ doc = "0x2d - UART CEA709.1-B Received Packet Length" ]
    pub rpl: Rpl,
    # [ doc = "0x2e - UART CEA709.1-B Received Preamble Length" ]
    pub rprel: Rprel,
    # [ doc = "0x2f - UART CEA709.1-B Collision Pulse Width" ]
    pub cpw: Cpw,
    # [ doc = "0x30 - UART CEA709.1-B Receive Indeterminate Time" ]
    pub ridt: Ridt,
    # [ doc = "0x31 - UART CEA709.1-B Transmit Indeterminate Time" ]
    pub tidt: Tidt,
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

# [ repr ( C ) ]
pub struct C7816 {
    register: ::volatile_register::RW<u8>,
}

impl C7816 {
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
        where for<'w> F: FnOnce(&C7816R, &'w mut C7816W) -> &'w mut C7816W
    {
        let bits = self.register.read();
        let r = C7816R { bits: bits };
        let mut w = C7816W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> C7816R {
        C7816R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut C7816W) -> &mut C7816W
    {
        let mut w = C7816W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct C7816R {
    bits: u8,
}

impl C7816R {
    # [ doc = "Bit 0 - ISO-7816 Functionality Enabled" ]
    pub fn iso_7816e(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Transfer Type" ]
    pub fn ttype(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Detect Initial Character" ]
    pub fn init(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Generate NACK on Error" ]
    pub fn anack(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Generate NACK on Overflow" ]
    pub fn onack(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct C7816W {
    bits: u8,
}

impl C7816W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        C7816W { bits: 0 }
    }
    # [ doc = "Bit 0 - ISO-7816 Functionality Enabled" ]
    pub fn iso_7816e(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Transfer Type" ]
    pub fn ttype(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Detect Initial Character" ]
    pub fn init(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Generate NACK on Error" ]
    pub fn anack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Generate NACK on Overflow" ]
    pub fn onack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Ie7816 {
    register: ::volatile_register::RW<u8>,
}

impl Ie7816 {
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
        where for<'w> F: FnOnce(&Ie7816R, &'w mut Ie7816W) -> &'w mut Ie7816W
    {
        let bits = self.register.read();
        let r = Ie7816R { bits: bits };
        let mut w = Ie7816W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ie7816R {
        Ie7816R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ie7816W) -> &mut Ie7816W
    {
        let mut w = Ie7816W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ie7816R {
    bits: u8,
}

impl Ie7816R {
    # [ doc = "Bit 0 - Receive Threshold Exceeded Interrupt Enable" ]
    pub fn rxte(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Transmit Threshold Exceeded Interrupt Enable" ]
    pub fn txte(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Guard Timer Violated Interrupt Enable" ]
    pub fn gtve(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Initial Character Detected Interrupt Enable" ]
    pub fn initde(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Block Wait Timer Interrupt Enable" ]
    pub fn bwte(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Character Wait Timer Interrupt Enable" ]
    pub fn cwte(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Wait Timer Interrupt Enable" ]
    pub fn wte(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ie7816W {
    bits: u8,
}

impl Ie7816W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ie7816W { bits: 0 }
    }
    # [ doc = "Bit 0 - Receive Threshold Exceeded Interrupt Enable" ]
    pub fn rxte(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Transmit Threshold Exceeded Interrupt Enable" ]
    pub fn txte(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Guard Timer Violated Interrupt Enable" ]
    pub fn gtve(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Initial Character Detected Interrupt Enable" ]
    pub fn initde(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Block Wait Timer Interrupt Enable" ]
    pub fn bwte(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Character Wait Timer Interrupt Enable" ]
    pub fn cwte(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Wait Timer Interrupt Enable" ]
    pub fn wte(&mut self, value: bool) -> &mut Self {
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
pub struct Is7816 {
    register: ::volatile_register::RW<u8>,
}

impl Is7816 {
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
        where for<'w> F: FnOnce(&Is7816R, &'w mut Is7816W) -> &'w mut Is7816W
    {
        let bits = self.register.read();
        let r = Is7816R { bits: bits };
        let mut w = Is7816W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Is7816R {
        Is7816R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Is7816W) -> &mut Is7816W
    {
        let mut w = Is7816W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Is7816R {
    bits: u8,
}

impl Is7816R {
    # [ doc = "Bit 0 - Receive Threshold Exceeded Interrupt" ]
    pub fn rxt(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Transmit Threshold Exceeded Interrupt" ]
    pub fn txt(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Guard Timer Violated Interrupt" ]
    pub fn gtv(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Initial Character Detected Interrupt" ]
    pub fn initd(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Block Wait Timer Interrupt" ]
    pub fn bwt(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Character Wait Timer Interrupt" ]
    pub fn cwt(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Wait Timer Interrupt" ]
    pub fn wt(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Is7816W {
    bits: u8,
}

impl Is7816W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Is7816W { bits: 0 }
    }
    # [ doc = "Bit 0 - Receive Threshold Exceeded Interrupt" ]
    pub fn rxt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Transmit Threshold Exceeded Interrupt" ]
    pub fn txt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Guard Timer Violated Interrupt" ]
    pub fn gtv(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Initial Character Detected Interrupt" ]
    pub fn initd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Block Wait Timer Interrupt" ]
    pub fn bwt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Character Wait Timer Interrupt" ]
    pub fn cwt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Wait Timer Interrupt" ]
    pub fn wt(&mut self, value: bool) -> &mut Self {
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
pub struct Wp7816t0 {
    register: ::volatile_register::RW<u8>,
}

impl Wp7816t0 {
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
        where for<'w> F: FnOnce(&Wp7816t0R, &'w mut Wp7816t0W) -> &'w mut Wp7816t0W
    {
        let bits = self.register.read();
        let r = Wp7816t0R { bits: bits };
        let mut w = Wp7816t0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Wp7816t0R {
        Wp7816t0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Wp7816t0W) -> &mut Wp7816t0W
    {
        let mut w = Wp7816t0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Wp7816t0R {
    bits: u8,
}

impl Wp7816t0R {
    # [ doc = "Bits 0:7 - Wait Timer Interrupt (C7816[TTYPE] = 0)" ]
    pub fn wi(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Wp7816t0W {
    bits: u8,
}

impl Wp7816t0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Wp7816t0W { bits: 10 }
    }
    # [ doc = "Bits 0:7 - Wait Timer Interrupt (C7816[TTYPE] = 0)" ]
    pub fn wi(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Wp7816t1 {
    register: ::volatile_register::RW<u8>,
}

impl Wp7816t1 {
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
        where for<'w> F: FnOnce(&Wp7816t1R, &'w mut Wp7816t1W) -> &'w mut Wp7816t1W
    {
        let bits = self.register.read();
        let r = Wp7816t1R { bits: bits };
        let mut w = Wp7816t1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Wp7816t1R {
        Wp7816t1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Wp7816t1W) -> &mut Wp7816t1W
    {
        let mut w = Wp7816t1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Wp7816t1R {
    bits: u8,
}

impl Wp7816t1R {
    # [ doc = "Bits 0:3 - Block Wait Time Integer(C7816[TTYPE] = 1)" ]
    pub fn bwi(&self) -> u8 {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - Character Wait Time Integer (C7816[TTYPE] = 1)" ]
    pub fn cwi(&self) -> u8 {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Wp7816t1W {
    bits: u8,
}

impl Wp7816t1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Wp7816t1W { bits: 10 }
    }
    # [ doc = "Bits 0:3 - Block Wait Time Integer(C7816[TTYPE] = 1)" ]
    pub fn bwi(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - Character Wait Time Integer (C7816[TTYPE] = 1)" ]
    pub fn cwi(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Wn7816 {
    register: ::volatile_register::RW<u8>,
}

impl Wn7816 {
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
        where for<'w> F: FnOnce(&Wn7816R, &'w mut Wn7816W) -> &'w mut Wn7816W
    {
        let bits = self.register.read();
        let r = Wn7816R { bits: bits };
        let mut w = Wn7816W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Wn7816R {
        Wn7816R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Wn7816W) -> &mut Wn7816W
    {
        let mut w = Wn7816W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Wn7816R {
    bits: u8,
}

impl Wn7816R {
    # [ doc = "Bits 0:7 - Guard Band N" ]
    pub fn gtn(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Wn7816W {
    bits: u8,
}

impl Wn7816W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Wn7816W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - Guard Band N" ]
    pub fn gtn(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Wf7816 {
    register: ::volatile_register::RW<u8>,
}

impl Wf7816 {
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
        where for<'w> F: FnOnce(&Wf7816R, &'w mut Wf7816W) -> &'w mut Wf7816W
    {
        let bits = self.register.read();
        let r = Wf7816R { bits: bits };
        let mut w = Wf7816W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Wf7816R {
        Wf7816R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Wf7816W) -> &mut Wf7816W
    {
        let mut w = Wf7816W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Wf7816R {
    bits: u8,
}

impl Wf7816R {
    # [ doc = "Bits 0:7 - FD Multiplier" ]
    pub fn gtfd(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Wf7816W {
    bits: u8,
}

impl Wf7816W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Wf7816W { bits: 1 }
    }
    # [ doc = "Bits 0:7 - FD Multiplier" ]
    pub fn gtfd(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Et7816 {
    register: ::volatile_register::RW<u8>,
}

impl Et7816 {
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
        where for<'w> F: FnOnce(&Et7816R, &'w mut Et7816W) -> &'w mut Et7816W
    {
        let bits = self.register.read();
        let r = Et7816R { bits: bits };
        let mut w = Et7816W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Et7816R {
        Et7816R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Et7816W) -> &mut Et7816W
    {
        let mut w = Et7816W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Et7816R {
    bits: u8,
}

impl Et7816R {
    # [ doc = "Bits 0:3 - Receive NACK Threshold" ]
    pub fn rxthreshold(&self) -> u8 {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - Transmit NACK Threshold" ]
    pub fn txthreshold(&self) -> u8 {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Et7816W {
    bits: u8,
}

impl Et7816W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Et7816W { bits: 0 }
    }
    # [ doc = "Bits 0:3 - Receive NACK Threshold" ]
    pub fn rxthreshold(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - Transmit NACK Threshold" ]
    pub fn txthreshold(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Tl7816 {
    register: ::volatile_register::RW<u8>,
}

impl Tl7816 {
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
        where for<'w> F: FnOnce(&Tl7816R, &'w mut Tl7816W) -> &'w mut Tl7816W
    {
        let bits = self.register.read();
        let r = Tl7816R { bits: bits };
        let mut w = Tl7816W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Tl7816R {
        Tl7816R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Tl7816W) -> &mut Tl7816W
    {
        let mut w = Tl7816W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Tl7816R {
    bits: u8,
}

impl Tl7816R {
    # [ doc = "Bits 0:7 - Transmit Length" ]
    pub fn tlen(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Tl7816W {
    bits: u8,
}

impl Tl7816W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Tl7816W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - Transmit Length" ]
    pub fn tlen(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct C6 {
    register: ::volatile_register::RW<u8>,
}

impl C6 {
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
        where for<'w> F: FnOnce(&C6R, &'w mut C6W) -> &'w mut C6W
    {
        let bits = self.register.read();
        let r = C6R { bits: bits };
        let mut w = C6W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> C6R {
        C6R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut C6W) -> &mut C6W
    {
        let mut w = C6W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct C6R {
    bits: u8,
}

impl C6R {
    # [ doc = "Bit 4 - Collision Signal Polarity" ]
    pub fn cp(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Collision Enable" ]
    pub fn ce(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - CEA709.1-B Transmit Enable" ]
    pub fn tx709(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - EN709" ]
    pub fn en709(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct C6W {
    bits: u8,
}

impl C6W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        C6W { bits: 0 }
    }
    # [ doc = "Bit 4 - Collision Signal Polarity" ]
    pub fn cp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Collision Enable" ]
    pub fn ce(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - CEA709.1-B Transmit Enable" ]
    pub fn tx709(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - EN709" ]
    pub fn en709(&mut self, value: bool) -> &mut Self {
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
pub struct Pcth {
    register: ::volatile_register::RW<u8>,
}

impl Pcth {
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
        where for<'w> F: FnOnce(&PcthR, &'w mut PcthW) -> &'w mut PcthW
    {
        let bits = self.register.read();
        let r = PcthR { bits: bits };
        let mut w = PcthW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PcthR {
        PcthR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PcthW) -> &mut PcthW
    {
        let mut w = PcthW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PcthR {
    bits: u8,
}

impl PcthR {
    # [ doc = "Bits 0:7 - Packet Cycle Time Counter High" ]
    pub fn pcth(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PcthW {
    bits: u8,
}

impl PcthW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PcthW { bits: 0 }
    }
    # [ doc = "Bits 0:7 - Packet Cycle Time Counter High" ]
    pub fn pcth(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pctl {
    register: ::volatile_register::RW<u8>,
}

impl Pctl {
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
        where for<'w> F: FnOnce(&PctlR, &'w mut PctlW) -> &'w mut PctlW
    {
        let bits = self.register.read();
        let r = PctlR { bits: bits };
        let mut w = PctlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PctlR {
        PctlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PctlW) -> &mut PctlW
    {
        let mut w = PctlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PctlR {
    bits: u8,
}

impl PctlR {
    # [ doc = "Bits 0:7 - Packet Cycle Time Counter Low" ]
    pub fn pctl(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PctlW {
    bits: u8,
}

impl PctlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PctlW { bits: 0 }
    }
    # [ doc = "Bits 0:7 - Packet Cycle Time Counter Low" ]
    pub fn pctl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct B1t {
    register: ::volatile_register::RW<u8>,
}

impl B1t {
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
        where for<'w> F: FnOnce(&B1tR, &'w mut B1tW) -> &'w mut B1tW
    {
        let bits = self.register.read();
        let r = B1tR { bits: bits };
        let mut w = B1tW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> B1tR {
        B1tR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut B1tW) -> &mut B1tW
    {
        let mut w = B1tW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct B1tR {
    bits: u8,
}

impl B1tR {
    # [ doc = "Bits 0:7 - Beta1 Timer" ]
    pub fn b1t(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct B1tW {
    bits: u8,
}

impl B1tW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        B1tW { bits: 0 }
    }
    # [ doc = "Bits 0:7 - Beta1 Timer" ]
    pub fn b1t(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Sdth {
    register: ::volatile_register::RW<u8>,
}

impl Sdth {
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
        where for<'w> F: FnOnce(&SdthR, &'w mut SdthW) -> &'w mut SdthW
    {
        let bits = self.register.read();
        let r = SdthR { bits: bits };
        let mut w = SdthW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> SdthR {
        SdthR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut SdthW) -> &mut SdthW
    {
        let mut w = SdthW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SdthR {
    bits: u8,
}

impl SdthR {
    # [ doc = "Bits 0:7 - Secondary Delay Timer High" ]
    pub fn sdth(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SdthW {
    bits: u8,
}

impl SdthW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        SdthW { bits: 0 }
    }
    # [ doc = "Bits 0:7 - Secondary Delay Timer High" ]
    pub fn sdth(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Sdtl {
    register: ::volatile_register::RW<u8>,
}

impl Sdtl {
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
        where for<'w> F: FnOnce(&SdtlR, &'w mut SdtlW) -> &'w mut SdtlW
    {
        let bits = self.register.read();
        let r = SdtlR { bits: bits };
        let mut w = SdtlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> SdtlR {
        SdtlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut SdtlW) -> &mut SdtlW
    {
        let mut w = SdtlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SdtlR {
    bits: u8,
}

impl SdtlR {
    # [ doc = "Bits 0:7 - Secondary Delay Timer Low" ]
    pub fn sdtl(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SdtlW {
    bits: u8,
}

impl SdtlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        SdtlW { bits: 0 }
    }
    # [ doc = "Bits 0:7 - Secondary Delay Timer Low" ]
    pub fn sdtl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pre {
    register: ::volatile_register::RW<u8>,
}

impl Pre {
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
        where for<'w> F: FnOnce(&PreR, &'w mut PreW) -> &'w mut PreW
    {
        let bits = self.register.read();
        let r = PreR { bits: bits };
        let mut w = PreW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PreR {
        PreR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PreW) -> &mut PreW
    {
        let mut w = PreW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PreR {
    bits: u8,
}

impl PreR {
    # [ doc = "Bits 0:7 - CEA709.1-B Preamble Register" ]
    pub fn preamble(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PreW {
    bits: u8,
}

impl PreW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PreW { bits: 0 }
    }
    # [ doc = "Bits 0:7 - CEA709.1-B Preamble Register" ]
    pub fn preamble(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Tpl {
    register: ::volatile_register::RW<u8>,
}

impl Tpl {
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
        where for<'w> F: FnOnce(&TplR, &'w mut TplW) -> &'w mut TplW
    {
        let bits = self.register.read();
        let r = TplR { bits: bits };
        let mut w = TplW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> TplR {
        TplR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut TplW) -> &mut TplW
    {
        let mut w = TplW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct TplR {
    bits: u8,
}

impl TplR {
    # [ doc = "Bits 0:7 - Transmit Packet Length Register" ]
    pub fn tpl(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct TplW {
    bits: u8,
}

impl TplW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        TplW { bits: 0 }
    }
    # [ doc = "Bits 0:7 - Transmit Packet Length Register" ]
    pub fn tpl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ie {
    register: ::volatile_register::RW<u8>,
}

impl Ie {
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
        where for<'w> F: FnOnce(&IeR, &'w mut IeW) -> &'w mut IeW
    {
        let bits = self.register.read();
        let r = IeR { bits: bits };
        let mut w = IeW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> IeR {
        IeR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut IeW) -> &mut IeW
    {
        let mut w = IeW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IeR {
    bits: u8,
}

impl IeR {
    # [ doc = "Bit 0 - Transmission Fail Interrupt Enable" ]
    pub fn txfie(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Preamble Start Interrupt Enable" ]
    pub fn psie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Packet Cycle Timer Interrupt Enable" ]
    pub fn pcteie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Packet Transmitted Interrupt Enable" ]
    pub fn ptxie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Packet Received Interrupt Enable" ]
    pub fn prxie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Initial Sync Detection Interrupt Enable" ]
    pub fn isdie(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Wbase Expired Interrupt Enable" ]
    pub fn wbeie(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IeW {
    bits: u8,
}

impl IeW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IeW { bits: 0 }
    }
    # [ doc = "Bit 0 - Transmission Fail Interrupt Enable" ]
    pub fn txfie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Preamble Start Interrupt Enable" ]
    pub fn psie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Packet Cycle Timer Interrupt Enable" ]
    pub fn pcteie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Packet Transmitted Interrupt Enable" ]
    pub fn ptxie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Packet Received Interrupt Enable" ]
    pub fn prxie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Initial Sync Detection Interrupt Enable" ]
    pub fn isdie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Wbase Expired Interrupt Enable" ]
    pub fn wbeie(&mut self, value: bool) -> &mut Self {
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
pub struct Wb {
    register: ::volatile_register::RW<u8>,
}

impl Wb {
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
        where for<'w> F: FnOnce(&WbR, &'w mut WbW) -> &'w mut WbW
    {
        let bits = self.register.read();
        let r = WbR { bits: bits };
        let mut w = WbW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> WbR {
        WbR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut WbW) -> &mut WbW
    {
        let mut w = WbW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct WbR {
    bits: u8,
}

impl WbR {
    # [ doc = "Bits 0:7 - CEA709.1-B WBASE register" ]
    pub fn wbase(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct WbW {
    bits: u8,
}

impl WbW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        WbW { bits: 0 }
    }
    # [ doc = "Bits 0:7 - CEA709.1-B WBASE register" ]
    pub fn wbase(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S3 {
    register: ::volatile_register::RW<u8>,
}

impl S3 {
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
        where for<'w> F: FnOnce(&S3R, &'w mut S3W) -> &'w mut S3W
    {
        let bits = self.register.read();
        let r = S3R { bits: bits };
        let mut w = S3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S3R {
        S3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S3W) -> &mut S3W
    {
        let mut w = S3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S3R {
    bits: u8,
}

impl S3R {
    # [ doc = "Bit 0 - Transmission Fail Flag" ]
    pub fn txff(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Preamble Start Flag" ]
    pub fn psf(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Packet Cycle Timer Expired Flag" ]
    pub fn pctef(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Packet Transmitted Flag" ]
    pub fn ptxf(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Packet Received Flag" ]
    pub fn prxf(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Initial Sync Detect" ]
    pub fn isd(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Wbase Expired Flag" ]
    pub fn wbef(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Preamble Error Flag" ]
    pub fn pef(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S3W {
    bits: u8,
}

impl S3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S3W { bits: 0 }
    }
    # [ doc = "Bit 0 - Transmission Fail Flag" ]
    pub fn txff(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Preamble Start Flag" ]
    pub fn psf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Packet Cycle Timer Expired Flag" ]
    pub fn pctef(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Packet Transmitted Flag" ]
    pub fn ptxf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Packet Received Flag" ]
    pub fn prxf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Wbase Expired Flag" ]
    pub fn wbef(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Preamble Error Flag" ]
    pub fn pef(&mut self, value: bool) -> &mut Self {
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
pub struct S4 {
    register: ::volatile_register::RW<u8>,
}

impl S4 {
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
        where for<'w> F: FnOnce(&S4R, &'w mut S4W) -> &'w mut S4W
    {
        let bits = self.register.read();
        let r = S4R { bits: bits };
        let mut w = S4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S4R {
        S4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S4W) -> &mut S4W
    {
        let mut w = S4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S4R {
    bits: u8,
}

impl S4R {
    # [ doc = "Bit 0 - Framing Error" ]
    pub fn fe(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Improper Line Code Violation" ]
    pub fn ilcv(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:3 - CDET" ]
    pub fn cdet(&self) -> u8 {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 4 - Initial Synchronization Fail Flag" ]
    pub fn initf(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S4W {
    bits: u8,
}

impl S4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S4W { bits: 0 }
    }
    # [ doc = "Bit 0 - Framing Error" ]
    pub fn fe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Improper Line Code Violation" ]
    pub fn ilcv(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:3 - CDET" ]
    pub fn cdet(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Rpl {
    register: ::volatile_register::RO<u8>,
}

impl Rpl {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub fn read(&self) -> RplR {
        RplR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct RplR {
    bits: u8,
}

impl RplR {
    # [ doc = "Bits 0:7 - Received packet length" ]
    pub fn rpl(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ repr ( C ) ]
pub struct Rprel {
    register: ::volatile_register::RO<u8>,
}

impl Rprel {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub fn read(&self) -> RprelR {
        RprelR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct RprelR {
    bits: u8,
}

impl RprelR {
    # [ doc = "Bits 0:7 - Received preamble length" ]
    pub fn rprel(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ repr ( C ) ]
pub struct Cpw {
    register: ::volatile_register::RW<u8>,
}

impl Cpw {
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
        where for<'w> F: FnOnce(&CpwR, &'w mut CpwW) -> &'w mut CpwW
    {
        let bits = self.register.read();
        let r = CpwR { bits: bits };
        let mut w = CpwW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CpwR {
        CpwR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CpwW) -> &mut CpwW
    {
        let mut w = CpwW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CpwR {
    bits: u8,
}

impl CpwR {
    # [ doc = "Bits 0:7 - CEA709.1-B CPW register" ]
    pub fn cpw(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CpwW {
    bits: u8,
}

impl CpwW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CpwW { bits: 0 }
    }
    # [ doc = "Bits 0:7 - CEA709.1-B CPW register" ]
    pub fn cpw(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ridt {
    register: ::volatile_register::RW<u8>,
}

impl Ridt {
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
        where for<'w> F: FnOnce(&RidtR, &'w mut RidtW) -> &'w mut RidtW
    {
        let bits = self.register.read();
        let r = RidtR { bits: bits };
        let mut w = RidtW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> RidtR {
        RidtR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut RidtW) -> &mut RidtW
    {
        let mut w = RidtW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct RidtR {
    bits: u8,
}

impl RidtR {
    # [ doc = "Bits 0:7 - CEA709.1-B Receive IDT Register" ]
    pub fn ridt(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct RidtW {
    bits: u8,
}

impl RidtW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        RidtW { bits: 0 }
    }
    # [ doc = "Bits 0:7 - CEA709.1-B Receive IDT Register" ]
    pub fn ridt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Tidt {
    register: ::volatile_register::RW<u8>,
}

impl Tidt {
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
        where for<'w> F: FnOnce(&TidtR, &'w mut TidtW) -> &'w mut TidtW
    {
        let bits = self.register.read();
        let r = TidtR { bits: bits };
        let mut w = TidtW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> TidtR {
        TidtR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut TidtW) -> &mut TidtW
    {
        let mut w = TidtW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct TidtR {
    bits: u8,
}

impl TidtR {
    # [ doc = "Bits 0:7 - CEA709.1-B Transmit IDT Register" ]
    pub fn tidt(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct TidtW {
    bits: u8,
}

impl TidtW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        TidtW { bits: 0 }
    }
    # [ doc = "Bits 0:7 - CEA709.1-B Transmit IDT Register" ]
    pub fn tidt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}
