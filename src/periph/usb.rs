# [ doc = "Universal Serial Bus, OTG Capable Controller" ]
# [ repr ( C ) ]
pub struct Usb {
    # [ doc = "0x00 - Peripheral ID Register" ]
    pub perid: Perid,
    _reserved0: [u8; 3usize],
    # [ doc = "0x04 - Peripheral ID Complement Register" ]
    pub idcomp: Idcomp,
    _reserved1: [u8; 3usize],
    # [ doc = "0x08 - Peripheral Revision Register" ]
    pub rev: Rev,
    _reserved2: [u8; 3usize],
    # [ doc = "0x0c - Peripheral Additional Info Register" ]
    pub addinfo: Addinfo,
    _reserved3: [u8; 3usize],
    # [ doc = "0x10 - OTG Interrupt Status Register" ]
    pub otgistat: Otgistat,
    _reserved4: [u8; 3usize],
    # [ doc = "0x14 - OTG Interrupt Control Register" ]
    pub otgicr: Otgicr,
    _reserved5: [u8; 3usize],
    # [ doc = "0x18 - OTG Status Register" ]
    pub otgstat: Otgstat,
    _reserved6: [u8; 3usize],
    # [ doc = "0x1c - OTG Control Register" ]
    pub otgctl: Otgctl,
    _reserved7: [u8; 99usize],
    # [ doc = "0x80 - Interrupt Status Register" ]
    pub istat: Istat,
    _reserved8: [u8; 3usize],
    # [ doc = "0x84 - Interrupt Enable Register" ]
    pub inten: Inten,
    _reserved9: [u8; 3usize],
    # [ doc = "0x88 - Error Interrupt Status Register" ]
    pub errstat: Errstat,
    _reserved10: [u8; 3usize],
    # [ doc = "0x8c - Error Interrupt Enable Register" ]
    pub erren: Erren,
    _reserved11: [u8; 3usize],
    # [ doc = "0x90 - Status Register" ]
    pub stat: Stat,
    _reserved12: [u8; 3usize],
    # [ doc = "0x94 - Control Register" ]
    pub ctl: Ctl,
    _reserved13: [u8; 3usize],
    # [ doc = "0x98 - Address Register" ]
    pub addr: Addr,
    _reserved14: [u8; 3usize],
    # [ doc = "0x9c - BDT Page Register 1" ]
    pub bdtpage1: Bdtpage1,
    _reserved15: [u8; 3usize],
    # [ doc = "0xa0 - Frame Number Register Low" ]
    pub frmnuml: Frmnuml,
    _reserved16: [u8; 3usize],
    # [ doc = "0xa4 - Frame Number Register High" ]
    pub frmnumh: Frmnumh,
    _reserved17: [u8; 3usize],
    # [ doc = "0xa8 - Token Register" ]
    pub token: Token,
    _reserved18: [u8; 3usize],
    # [ doc = "0xac - SOF Threshold Register" ]
    pub softhld: Softhld,
    _reserved19: [u8; 3usize],
    # [ doc = "0xb0 - BDT Page Register 2" ]
    pub bdtpage2: Bdtpage2,
    _reserved20: [u8; 3usize],
    # [ doc = "0xb4 - BDT Page Register 3" ]
    pub bdtpage3: Bdtpage3,
    _reserved21: [u8; 11usize],
    # [ doc = "0xc0 - Endpoint Control Register" ]
    pub endpt0: Endpt,
    _reserved22: [u8; 3usize],
    # [ doc = "0xc4 - Endpoint Control Register" ]
    pub endpt1: Endpt,
    _reserved23: [u8; 3usize],
    # [ doc = "0xc8 - Endpoint Control Register" ]
    pub endpt2: Endpt,
    _reserved24: [u8; 3usize],
    # [ doc = "0xcc - Endpoint Control Register" ]
    pub endpt3: Endpt,
    _reserved25: [u8; 3usize],
    # [ doc = "0xd0 - Endpoint Control Register" ]
    pub endpt4: Endpt,
    _reserved26: [u8; 3usize],
    # [ doc = "0xd4 - Endpoint Control Register" ]
    pub endpt5: Endpt,
    _reserved27: [u8; 3usize],
    # [ doc = "0xd8 - Endpoint Control Register" ]
    pub endpt6: Endpt,
    _reserved28: [u8; 3usize],
    # [ doc = "0xdc - Endpoint Control Register" ]
    pub endpt7: Endpt,
    _reserved29: [u8; 3usize],
    # [ doc = "0xe0 - Endpoint Control Register" ]
    pub endpt8: Endpt,
    _reserved30: [u8; 3usize],
    # [ doc = "0xe4 - Endpoint Control Register" ]
    pub endpt9: Endpt,
    _reserved31: [u8; 3usize],
    # [ doc = "0xe8 - Endpoint Control Register" ]
    pub endpt10: Endpt,
    _reserved32: [u8; 3usize],
    # [ doc = "0xec - Endpoint Control Register" ]
    pub endpt11: Endpt,
    _reserved33: [u8; 3usize],
    # [ doc = "0xf0 - Endpoint Control Register" ]
    pub endpt12: Endpt,
    _reserved34: [u8; 3usize],
    # [ doc = "0xf4 - Endpoint Control Register" ]
    pub endpt13: Endpt,
    _reserved35: [u8; 3usize],
    # [ doc = "0xf8 - Endpoint Control Register" ]
    pub endpt14: Endpt,
    _reserved36: [u8; 3usize],
    # [ doc = "0xfc - Endpoint Control Register" ]
    pub endpt15: Endpt,
    _reserved37: [u8; 3usize],
    # [ doc = "0x100 - USB Control Register" ]
    pub usbctrl: Usbctrl,
    _reserved38: [u8; 3usize],
    # [ doc = "0x104 - USB OTG Observe Register" ]
    pub observe: Observe,
    _reserved39: [u8; 3usize],
    # [ doc = "0x108 - USB OTG Control Register" ]
    pub control: Control,
    _reserved40: [u8; 3usize],
    # [ doc = "0x10c - USB Transceiver Control Register 0" ]
    pub usbtrc0: Usbtrc0,
    _reserved41: [u8; 7usize],
    # [ doc = "0x114 - Frame Adjust Register" ]
    pub usbfrmadjust: Usbfrmadjust,
}

# [ repr ( C ) ]
pub struct Perid {
    register: ::volatile_register::RO<u8>,
}

impl Perid {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub fn read(&self) -> PeridR {
        PeridR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PeridR {
    bits: u8,
}

impl PeridR {
    # [ doc = "Bits 0:5 - Peripheral identification bits" ]
    pub fn id(&self) -> u8 {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ repr ( C ) ]
pub struct Idcomp {
    register: ::volatile_register::RO<u8>,
}

impl Idcomp {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub fn read(&self) -> IdcompR {
        IdcompR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IdcompR {
    bits: u8,
}

impl IdcompR {
    # [ doc = "Bits 0:5 - Ones complement of peripheral identification bits." ]
    pub fn nid(&self) -> u8 {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ repr ( C ) ]
pub struct Rev {
    register: ::volatile_register::RO<u8>,
}

impl Rev {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub fn read(&self) -> RevR {
        RevR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct RevR {
    bits: u8,
}

impl RevR {
    # [ doc = "Bits 0:7 - Revision" ]
    pub fn rev(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ repr ( C ) ]
pub struct Addinfo {
    register: ::volatile_register::RO<u8>,
}

impl Addinfo {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub fn read(&self) -> AddinfoR {
        AddinfoR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AddinfoR {
    bits: u8,
}

impl AddinfoR {
    # [ doc = "Bit 0 - This bit is set if host mode is enabled." ]
    pub fn iehost(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 3:7 - Assigned Interrupt Request Number" ]
    pub fn irqnum(&self) -> u8 {
        const MASK: u8 = 31;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ repr ( C ) ]
pub struct Otgistat {
    register: ::volatile_register::RW<u8>,
}

impl Otgistat {
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
        where for<'w> F: FnOnce(&OtgistatR, &'w mut OtgistatW) -> &'w mut OtgistatW
    {
        let bits = self.register.read();
        let r = OtgistatR { bits: bits };
        let mut w = OtgistatW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgistatR {
        OtgistatR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgistatW) -> &mut OtgistatW
    {
        let mut w = OtgistatW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgistatR {
    bits: u8,
}

impl OtgistatR {
    # [ doc = "Bit 0 - This bit is set when a change in VBUS is detected on an A device." ]
    pub fn avbuschg(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - This bit is set when a change in VBUS is detected on a B device." ]
    pub fn b_sess_chg(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - This bit is set when a change in VBUS is detected indicating a session valid or a session no longer valid" ]
    pub fn sessvldchg(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - This bit is set when the USB line state changes" ]
    pub fn line_state_chg(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - This bit is set when the 1 millisecond timer expires" ]
    pub fn onemsec(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - This bit is set when a change in the ID Signal from the USB connector is sensed." ]
    pub fn idchg(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgistatW {
    bits: u8,
}

impl OtgistatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgistatW { bits: 0 }
    }
    # [ doc = "Bit 0 - This bit is set when a change in VBUS is detected on an A device." ]
    pub fn avbuschg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - This bit is set when a change in VBUS is detected on a B device." ]
    pub fn b_sess_chg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - This bit is set when a change in VBUS is detected indicating a session valid or a session no longer valid" ]
    pub fn sessvldchg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - This bit is set when the USB line state changes" ]
    pub fn line_state_chg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - This bit is set when the 1 millisecond timer expires" ]
    pub fn onemsec(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - This bit is set when a change in the ID Signal from the USB connector is sensed." ]
    pub fn idchg(&mut self, value: bool) -> &mut Self {
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
pub struct Otgicr {
    register: ::volatile_register::RW<u8>,
}

impl Otgicr {
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
        where for<'w> F: FnOnce(&OtgicrR, &'w mut OtgicrW) -> &'w mut OtgicrW
    {
        let bits = self.register.read();
        let r = OtgicrR { bits: bits };
        let mut w = OtgicrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgicrR {
        OtgicrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgicrW) -> &mut OtgicrW
    {
        let mut w = OtgicrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgicrR {
    bits: u8,
}

impl OtgicrR {
    # [ doc = "Bit 0 - A VBUS Valid interrupt enable" ]
    pub fn avbusen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - B Session END interrupt enable" ]
    pub fn bsessen(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Session valid interrupt enable" ]
    pub fn sessvlden(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Line State change interrupt enable" ]
    pub fn linestateen(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - 1 millisecond interrupt enable" ]
    pub fn onemsecen(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - ID interrupt enable" ]
    pub fn iden(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgicrW {
    bits: u8,
}

impl OtgicrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgicrW { bits: 0 }
    }
    # [ doc = "Bit 0 - A VBUS Valid interrupt enable" ]
    pub fn avbusen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - B Session END interrupt enable" ]
    pub fn bsessen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Session valid interrupt enable" ]
    pub fn sessvlden(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Line State change interrupt enable" ]
    pub fn linestateen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - 1 millisecond interrupt enable" ]
    pub fn onemsecen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - ID interrupt enable" ]
    pub fn iden(&mut self, value: bool) -> &mut Self {
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
pub struct Otgstat {
    register: ::volatile_register::RW<u8>,
}

impl Otgstat {
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
        where for<'w> F: FnOnce(&OtgstatR, &'w mut OtgstatW) -> &'w mut OtgstatW
    {
        let bits = self.register.read();
        let r = OtgstatR { bits: bits };
        let mut w = OtgstatW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgstatR {
        OtgstatR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgstatW) -> &mut OtgstatW
    {
        let mut w = OtgstatW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgstatR {
    bits: u8,
}

impl OtgstatR {
    # [ doc = "Bit 0 - A VBUS Valid" ]
    pub fn avbusvld(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - B Session END" ]
    pub fn bsessend(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Session valid" ]
    pub fn sess_vld(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - This bit indicates that the internal signals that control the LINE_STATE_CHG bit (bit 5) of the OTGISTAT register have been stable for at least 1 millisecond" ]
    pub fn linestatestable(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - This bit is reserved for the 1msec count, but it is not useful to software." ]
    pub fn onemsecen(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Indicates the current state of the ID pin on the USB connector" ]
    pub fn id(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgstatW {
    bits: u8,
}

impl OtgstatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgstatW { bits: 0 }
    }
    # [ doc = "Bit 0 - A VBUS Valid" ]
    pub fn avbusvld(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - B Session END" ]
    pub fn bsessend(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Session valid" ]
    pub fn sess_vld(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - This bit indicates that the internal signals that control the LINE_STATE_CHG bit (bit 5) of the OTGISTAT register have been stable for at least 1 millisecond" ]
    pub fn linestatestable(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - This bit is reserved for the 1msec count, but it is not useful to software." ]
    pub fn onemsecen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Indicates the current state of the ID pin on the USB connector" ]
    pub fn id(&mut self, value: bool) -> &mut Self {
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
pub struct Otgctl {
    register: ::volatile_register::RW<u8>,
}

impl Otgctl {
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
        where for<'w> F: FnOnce(&OtgctlR, &'w mut OtgctlW) -> &'w mut OtgctlW
    {
        let bits = self.register.read();
        let r = OtgctlR { bits: bits };
        let mut w = OtgctlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtgctlR {
        OtgctlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtgctlW) -> &mut OtgctlW
    {
        let mut w = OtgctlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgctlR {
    bits: u8,
}

impl OtgctlR {
    # [ doc = "Bit 2 - On-The-Go pullup/pulldown resistor enable" ]
    pub fn otgen(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - D- Data Line pull-down resistor enable" ]
    pub fn dmlow(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - D+ Data Line pull-down resistor enable" ]
    pub fn dplow(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - D+ Data Line pullup resistor enable" ]
    pub fn dphigh(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OtgctlW {
    bits: u8,
}

impl OtgctlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OtgctlW { bits: 0 }
    }
    # [ doc = "Bit 2 - On-The-Go pullup/pulldown resistor enable" ]
    pub fn otgen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - D- Data Line pull-down resistor enable" ]
    pub fn dmlow(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - D+ Data Line pull-down resistor enable" ]
    pub fn dplow(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - D+ Data Line pullup resistor enable" ]
    pub fn dphigh(&mut self, value: bool) -> &mut Self {
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
pub struct Istat {
    register: ::volatile_register::RW<u8>,
}

impl Istat {
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
        where for<'w> F: FnOnce(&IstatR, &'w mut IstatW) -> &'w mut IstatW
    {
        let bits = self.register.read();
        let r = IstatR { bits: bits };
        let mut w = IstatW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> IstatR {
        IstatR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut IstatW) -> &mut IstatW
    {
        let mut w = IstatW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IstatR {
    bits: u8,
}

impl IstatR {
    # [ doc = "Bit 0 - This bit is set when the USB Module has decoded a valid USB reset" ]
    pub fn usbrst(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - This bit is set when any of the error conditions within the ERRSTAT register occur" ]
    pub fn error(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - This bit is set when the USB Module receives a Start Of Frame (SOF) token" ]
    pub fn softok(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - This bit is set when the current token being processed has completed" ]
    pub fn tokdne(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - This bit is set when the USB Module detects a constant idle on the USB bus for 3 milliseconds" ]
    pub fn sleep(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - This bit is set depending upon the DP/DM signals, and can be used to signal remote wake-up signaling on the USB bus" ]
    pub fn resume(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Attach Interrupt" ]
    pub fn attach(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Stall Interrupt" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IstatW {
    bits: u8,
}

impl IstatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IstatW { bits: 0 }
    }
    # [ doc = "Bit 0 - This bit is set when the USB Module has decoded a valid USB reset" ]
    pub fn usbrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - This bit is set when any of the error conditions within the ERRSTAT register occur" ]
    pub fn error(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - This bit is set when the USB Module receives a Start Of Frame (SOF) token" ]
    pub fn softok(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - This bit is set when the current token being processed has completed" ]
    pub fn tokdne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - This bit is set when the USB Module detects a constant idle on the USB bus for 3 milliseconds" ]
    pub fn sleep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - This bit is set depending upon the DP/DM signals, and can be used to signal remote wake-up signaling on the USB bus" ]
    pub fn resume(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Attach Interrupt" ]
    pub fn attach(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Stall Interrupt" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
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
pub struct Inten {
    register: ::volatile_register::RW<u8>,
}

impl Inten {
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
        where for<'w> F: FnOnce(&IntenR, &'w mut IntenW) -> &'w mut IntenW
    {
        let bits = self.register.read();
        let r = IntenR { bits: bits };
        let mut w = IntenW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> IntenR {
        IntenR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut IntenW) -> &mut IntenW
    {
        let mut w = IntenW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IntenR {
    bits: u8,
}

impl IntenR {
    # [ doc = "Bit 0 - USBRST Interrupt Enable" ]
    pub fn usbrsten(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - ERROR Interrupt Enable" ]
    pub fn erroren(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - SOFTOK Interrupt Enable" ]
    pub fn softoken(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - TOKDNE Interrupt Enable" ]
    pub fn tokdneen(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - SLEEP Interrupt Enable" ]
    pub fn sleepen(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - RESUME Interrupt Enable" ]
    pub fn resumeen(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - ATTACH Interrupt Enable" ]
    pub fn attachen(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - STALL Interrupt Enable" ]
    pub fn stallen(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IntenW {
    bits: u8,
}

impl IntenW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IntenW { bits: 0 }
    }
    # [ doc = "Bit 0 - USBRST Interrupt Enable" ]
    pub fn usbrsten(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - ERROR Interrupt Enable" ]
    pub fn erroren(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - SOFTOK Interrupt Enable" ]
    pub fn softoken(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - TOKDNE Interrupt Enable" ]
    pub fn tokdneen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - SLEEP Interrupt Enable" ]
    pub fn sleepen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - RESUME Interrupt Enable" ]
    pub fn resumeen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - ATTACH Interrupt Enable" ]
    pub fn attachen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - STALL Interrupt Enable" ]
    pub fn stallen(&mut self, value: bool) -> &mut Self {
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
pub struct Errstat {
    register: ::volatile_register::RW<u8>,
}

impl Errstat {
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
        where for<'w> F: FnOnce(&ErrstatR, &'w mut ErrstatW) -> &'w mut ErrstatW
    {
        let bits = self.register.read();
        let r = ErrstatR { bits: bits };
        let mut w = ErrstatW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ErrstatR {
        ErrstatR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ErrstatW) -> &mut ErrstatW
    {
        let mut w = ErrstatW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ErrstatR {
    bits: u8,
}

impl ErrstatR {
    # [ doc = "Bit 0 - This bit is set when the PID check field fails." ]
    pub fn piderr(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - This error interrupt has two functions" ]
    pub fn crc5eof(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - This bit is set when a data packet is rejected due to a CRC16 error." ]
    pub fn crc16(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - This bit is set if the data field received was not 8 bits in length" ]
    pub fn dfn8(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - This bit is set when a bus turnaround timeout error occurs" ]
    pub fn btoerr(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - This bit is set if the USB Module has requested a DMA access to read a new BDT but has not been given the bus before it needs to receive or transmit data" ]
    pub fn dmaerr(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - This bit is set when a bit stuff error is detected" ]
    pub fn btserr(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ErrstatW {
    bits: u8,
}

impl ErrstatW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ErrstatW { bits: 0 }
    }
    # [ doc = "Bit 0 - This bit is set when the PID check field fails." ]
    pub fn piderr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - This error interrupt has two functions" ]
    pub fn crc5eof(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - This bit is set when a data packet is rejected due to a CRC16 error." ]
    pub fn crc16(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - This bit is set if the data field received was not 8 bits in length" ]
    pub fn dfn8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - This bit is set when a bus turnaround timeout error occurs" ]
    pub fn btoerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - This bit is set if the USB Module has requested a DMA access to read a new BDT but has not been given the bus before it needs to receive or transmit data" ]
    pub fn dmaerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - This bit is set when a bit stuff error is detected" ]
    pub fn btserr(&mut self, value: bool) -> &mut Self {
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
pub struct Erren {
    register: ::volatile_register::RW<u8>,
}

impl Erren {
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
        where for<'w> F: FnOnce(&ErrenR, &'w mut ErrenW) -> &'w mut ErrenW
    {
        let bits = self.register.read();
        let r = ErrenR { bits: bits };
        let mut w = ErrenW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ErrenR {
        ErrenR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ErrenW) -> &mut ErrenW
    {
        let mut w = ErrenW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ErrenR {
    bits: u8,
}

impl ErrenR {
    # [ doc = "Bit 0 - PIDERR Interrupt Enable" ]
    pub fn piderren(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - CRC5/EOF Interrupt Enable" ]
    pub fn crc5eofen(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - CRC16 Interrupt Enable" ]
    pub fn crc16en(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - DFN8 Interrupt Enable" ]
    pub fn dfn8en(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - BTOERR Interrupt Enable" ]
    pub fn btoerren(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - DMAERR Interrupt Enable" ]
    pub fn dmaerren(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - BTSERR Interrupt Enable" ]
    pub fn btserren(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ErrenW {
    bits: u8,
}

impl ErrenW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ErrenW { bits: 0 }
    }
    # [ doc = "Bit 0 - PIDERR Interrupt Enable" ]
    pub fn piderren(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - CRC5/EOF Interrupt Enable" ]
    pub fn crc5eofen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - CRC16 Interrupt Enable" ]
    pub fn crc16en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - DFN8 Interrupt Enable" ]
    pub fn dfn8en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - BTOERR Interrupt Enable" ]
    pub fn btoerren(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - DMAERR Interrupt Enable" ]
    pub fn dmaerren(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - BTSERR Interrupt Enable" ]
    pub fn btserren(&mut self, value: bool) -> &mut Self {
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
pub struct Stat {
    register: ::volatile_register::RO<u8>,
}

impl Stat {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub fn read(&self) -> StatR {
        StatR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct StatR {
    bits: u8,
}

impl StatR {
    # [ doc = "Bit 2 - this bit is set if the last Buffer Descriptor updated was in the odd bank of the BDT." ]
    pub fn odd(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Transmit Indicator" ]
    pub fn tx(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:7 - This four-bit field encodes the endpoint address that received or transmitted the previous token" ]
    pub fn endp(&self) -> u8 {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ repr ( C ) ]
pub struct Ctl {
    register: ::volatile_register::RW<u8>,
}

impl Ctl {
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
        where for<'w> F: FnOnce(&CtlR, &'w mut CtlW) -> &'w mut CtlW
    {
        let bits = self.register.read();
        let r = CtlR { bits: bits };
        let mut w = CtlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CtlR {
        CtlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CtlW) -> &mut CtlW
    {
        let mut w = CtlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CtlR {
    bits: u8,
}

impl CtlR {
    # [ doc = "Bit 0 - USB Enable" ]
    pub fn usbensofen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Setting this bit to 1 resets all the BDT ODD ping/pong bits to 0, which then specifies the EVEN BDT bank" ]
    pub fn oddrst(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - When set to 1 this bit enables the USB Module to execute resume signaling" ]
    pub fn resume(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - When set to 1, this bit enables the USB Module to operate in Host mode" ]
    pub fn hostmodeen(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Setting this bit enables the USB Module to generate USB reset signaling" ]
    pub fn reset(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - When the USB Module is in Host mode TOKEN_BUSY is set when the USB Module is busy executing a USB token and no more token commands should be written to the Token Register" ]
    pub fn txsuspendtokenbusy(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Live USB Single Ended Zero signal" ]
    pub fn se0(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Live USB differential receiver JSTATE signal" ]
    pub fn jstate(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CtlW {
    bits: u8,
}

impl CtlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CtlW { bits: 0 }
    }
    # [ doc = "Bit 0 - USB Enable" ]
    pub fn usbensofen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Setting this bit to 1 resets all the BDT ODD ping/pong bits to 0, which then specifies the EVEN BDT bank" ]
    pub fn oddrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - When set to 1 this bit enables the USB Module to execute resume signaling" ]
    pub fn resume(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - When set to 1, this bit enables the USB Module to operate in Host mode" ]
    pub fn hostmodeen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Setting this bit enables the USB Module to generate USB reset signaling" ]
    pub fn reset(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - When the USB Module is in Host mode TOKEN_BUSY is set when the USB Module is busy executing a USB token and no more token commands should be written to the Token Register" ]
    pub fn txsuspendtokenbusy(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Live USB Single Ended Zero signal" ]
    pub fn se0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Live USB differential receiver JSTATE signal" ]
    pub fn jstate(&mut self, value: bool) -> &mut Self {
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
pub struct Addr {
    register: ::volatile_register::RW<u8>,
}

impl Addr {
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
        where for<'w> F: FnOnce(&AddrR, &'w mut AddrW) -> &'w mut AddrW
    {
        let bits = self.register.read();
        let r = AddrR { bits: bits };
        let mut w = AddrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> AddrR {
        AddrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut AddrW) -> &mut AddrW
    {
        let mut w = AddrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AddrR {
    bits: u8,
}

impl AddrR {
    # [ doc = "Bits 0:6 - USB address" ]
    pub fn addr(&self) -> u8 {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - Low Speed Enable bit" ]
    pub fn lsen(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AddrW {
    bits: u8,
}

impl AddrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        AddrW { bits: 0 }
    }
    # [ doc = "Bits 0:6 - USB address" ]
    pub fn addr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Low Speed Enable bit" ]
    pub fn lsen(&mut self, value: bool) -> &mut Self {
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
pub struct Bdtpage1 {
    register: ::volatile_register::RW<u8>,
}

impl Bdtpage1 {
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
        where for<'w> F: FnOnce(&Bdtpage1R, &'w mut Bdtpage1W) -> &'w mut Bdtpage1W
    {
        let bits = self.register.read();
        let r = Bdtpage1R { bits: bits };
        let mut w = Bdtpage1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bdtpage1R {
        Bdtpage1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bdtpage1W) -> &mut Bdtpage1W
    {
        let mut w = Bdtpage1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bdtpage1R {
    bits: u8,
}

impl Bdtpage1R {
    # [ doc = "Bits 1:7 - This field provides address bits 15 through 9 of the BDT base address." ]
    pub fn bdtba(&self) -> u8 {
        const MASK: u8 = 127;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bdtpage1W {
    bits: u8,
}

impl Bdtpage1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bdtpage1W { bits: 0 }
    }
    # [ doc = "Bits 1:7 - This field provides address bits 15 through 9 of the BDT base address." ]
    pub fn bdtba(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Frmnuml {
    register: ::volatile_register::RW<u8>,
}

impl Frmnuml {
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
        where for<'w> F: FnOnce(&FrmnumlR, &'w mut FrmnumlW) -> &'w mut FrmnumlW
    {
        let bits = self.register.read();
        let r = FrmnumlR { bits: bits };
        let mut w = FrmnumlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FrmnumlR {
        FrmnumlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FrmnumlW) -> &mut FrmnumlW
    {
        let mut w = FrmnumlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FrmnumlR {
    bits: u8,
}

impl FrmnumlR {
    # [ doc = "Bits 0:7 - This 8-bit field and the 3-bit field in the Frame Number Register High are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory" ]
    pub fn frm(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FrmnumlW {
    bits: u8,
}

impl FrmnumlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FrmnumlW { bits: 0 }
    }
    # [ doc = "Bits 0:7 - This 8-bit field and the 3-bit field in the Frame Number Register High are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory" ]
    pub fn frm(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Frmnumh {
    register: ::volatile_register::RW<u8>,
}

impl Frmnumh {
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
        where for<'w> F: FnOnce(&FrmnumhR, &'w mut FrmnumhW) -> &'w mut FrmnumhW
    {
        let bits = self.register.read();
        let r = FrmnumhR { bits: bits };
        let mut w = FrmnumhW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FrmnumhR {
        FrmnumhR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FrmnumhW) -> &mut FrmnumhW
    {
        let mut w = FrmnumhW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FrmnumhR {
    bits: u8,
}

impl FrmnumhR {
    # [ doc = "Bits 0:2 - This 3-bit field and the 8-bit field in the Frame Number Register Low are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory" ]
    pub fn frm(&self) -> u8 {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FrmnumhW {
    bits: u8,
}

impl FrmnumhW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FrmnumhW { bits: 0 }
    }
    # [ doc = "Bits 0:2 - This 3-bit field and the 8-bit field in the Frame Number Register Low are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory" ]
    pub fn frm(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Token {
    register: ::volatile_register::RW<u8>,
}

impl Token {
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
        where for<'w> F: FnOnce(&TokenR, &'w mut TokenW) -> &'w mut TokenW
    {
        let bits = self.register.read();
        let r = TokenR { bits: bits };
        let mut w = TokenW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> TokenR {
        TokenR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut TokenW) -> &mut TokenW
    {
        let mut w = TokenW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct TokenR {
    bits: u8,
}

impl TokenR {
    # [ doc = "Bits 0:3 - This 4 bit field holds the Endpoint address for the token command" ]
    pub fn tokenendpt(&self) -> u8 {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - This 4-bit field contains the token type executed by the USB Module." ]
    pub fn tokenpid(&self) -> u8 {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct TokenW {
    bits: u8,
}

impl TokenW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        TokenW { bits: 0 }
    }
    # [ doc = "Bits 0:3 - This 4 bit field holds the Endpoint address for the token command" ]
    pub fn tokenendpt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - This 4-bit field contains the token type executed by the USB Module." ]
    pub fn tokenpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Softhld {
    register: ::volatile_register::RW<u8>,
}

impl Softhld {
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
        where for<'w> F: FnOnce(&SofthldR, &'w mut SofthldW) -> &'w mut SofthldW
    {
        let bits = self.register.read();
        let r = SofthldR { bits: bits };
        let mut w = SofthldW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> SofthldR {
        SofthldR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut SofthldW) -> &mut SofthldW
    {
        let mut w = SofthldW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SofthldR {
    bits: u8,
}

impl SofthldR {
    # [ doc = "Bits 0:7 - This 8-bit field represents the SOF count threshold in byte times." ]
    pub fn cnt(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SofthldW {
    bits: u8,
}

impl SofthldW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        SofthldW { bits: 0 }
    }
    # [ doc = "Bits 0:7 - This 8-bit field represents the SOF count threshold in byte times." ]
    pub fn cnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bdtpage2 {
    register: ::volatile_register::RW<u8>,
}

impl Bdtpage2 {
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
        where for<'w> F: FnOnce(&Bdtpage2R, &'w mut Bdtpage2W) -> &'w mut Bdtpage2W
    {
        let bits = self.register.read();
        let r = Bdtpage2R { bits: bits };
        let mut w = Bdtpage2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bdtpage2R {
        Bdtpage2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bdtpage2W) -> &mut Bdtpage2W
    {
        let mut w = Bdtpage2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bdtpage2R {
    bits: u8,
}

impl Bdtpage2R {
    # [ doc = "Bits 0:7 - This 8 bit field provides address bits 23 through 16 of the BDT base address, which defines where the Buffer Descriptor Table resides in system memory" ]
    pub fn bdtba(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bdtpage2W {
    bits: u8,
}

impl Bdtpage2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bdtpage2W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - This 8 bit field provides address bits 23 through 16 of the BDT base address, which defines where the Buffer Descriptor Table resides in system memory" ]
    pub fn bdtba(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bdtpage3 {
    register: ::volatile_register::RW<u8>,
}

impl Bdtpage3 {
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
        where for<'w> F: FnOnce(&Bdtpage3R, &'w mut Bdtpage3W) -> &'w mut Bdtpage3W
    {
        let bits = self.register.read();
        let r = Bdtpage3R { bits: bits };
        let mut w = Bdtpage3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bdtpage3R {
        Bdtpage3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bdtpage3W) -> &mut Bdtpage3W
    {
        let mut w = Bdtpage3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bdtpage3R {
    bits: u8,
}

impl Bdtpage3R {
    # [ doc = "Bits 0:7 - This 8 bit field provides address bits 31 through 24 of the BDT base address, which defines where the Buffer Descriptor Table resides in system memory" ]
    pub fn bdtba(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bdtpage3W {
    bits: u8,
}

impl Bdtpage3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bdtpage3W { bits: 0 }
    }
    # [ doc = "Bits 0:7 - This 8 bit field provides address bits 31 through 24 of the BDT base address, which defines where the Buffer Descriptor Table resides in system memory" ]
    pub fn bdtba(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Endpt {
    register: ::volatile_register::RW<u8>,
}

impl Endpt {
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
        where for<'w> F: FnOnce(&EndptR, &'w mut EndptW) -> &'w mut EndptW
    {
        let bits = self.register.read();
        let r = EndptR { bits: bits };
        let mut w = EndptW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> EndptR {
        EndptR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut EndptW) -> &mut EndptW
    {
        let mut w = EndptW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct EndptR {
    bits: u8,
}

impl EndptR {
    # [ doc = "Bit 0 - no description available" ]
    pub fn ephshk(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - no description available" ]
    pub fn epstall(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - no description available" ]
    pub fn eptxen(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - no description available" ]
    pub fn eprxen(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - no description available" ]
    pub fn epctldis(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - no description available" ]
    pub fn retrydis(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - no description available" ]
    pub fn hostwohub(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct EndptW {
    bits: u8,
}

impl EndptW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        EndptW { bits: 0 }
    }
    # [ doc = "Bit 0 - no description available" ]
    pub fn ephshk(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - no description available" ]
    pub fn epstall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - no description available" ]
    pub fn eptxen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - no description available" ]
    pub fn eprxen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - no description available" ]
    pub fn epctldis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - no description available" ]
    pub fn retrydis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - no description available" ]
    pub fn hostwohub(&mut self, value: bool) -> &mut Self {
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
pub struct Usbctrl {
    register: ::volatile_register::RW<u8>,
}

impl Usbctrl {
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
        where for<'w> F: FnOnce(&UsbctrlR, &'w mut UsbctrlW) -> &'w mut UsbctrlW
    {
        let bits = self.register.read();
        let r = UsbctrlR { bits: bits };
        let mut w = UsbctrlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> UsbctrlR {
        UsbctrlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut UsbctrlW) -> &mut UsbctrlW
    {
        let mut w = UsbctrlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct UsbctrlR {
    bits: u8,
}

impl UsbctrlR {
    # [ doc = "Bit 6 - Enables the weak pulldowns on the USB transceiver." ]
    pub fn pde(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Places the USB transceiver into the suspend state." ]
    pub fn susp(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct UsbctrlW {
    bits: u8,
}

impl UsbctrlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        UsbctrlW { bits: 192 }
    }
    # [ doc = "Bit 6 - Enables the weak pulldowns on the USB transceiver." ]
    pub fn pde(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Places the USB transceiver into the suspend state." ]
    pub fn susp(&mut self, value: bool) -> &mut Self {
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
pub struct Observe {
    register: ::volatile_register::RO<u8>,
}

impl Observe {
    pub fn read_bits(&self) -> u8 {
        self.register.read()
    }
    pub fn read(&self) -> ObserveR {
        ObserveR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ObserveR {
    bits: u8,
}

impl ObserveR {
    # [ doc = "Bit 4 - Provides observability of the D- Pull Down enable at the USB transceiver." ]
    pub fn dmpd(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Provides observability of the D+ Pull Down enable at the USB transceiver." ]
    pub fn dppd(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Provides observability of the D+ Pull Up enable at the USB transceiver." ]
    pub fn dppu(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ repr ( C ) ]
pub struct Control {
    register: ::volatile_register::RW<u8>,
}

impl Control {
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
        where for<'w> F: FnOnce(&ControlR, &'w mut ControlW) -> &'w mut ControlW
    {
        let bits = self.register.read();
        let r = ControlR { bits: bits };
        let mut w = ControlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ControlR {
        ControlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ControlW) -> &mut ControlW
    {
        let mut w = ControlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ControlR {
    bits: u8,
}

impl ControlR {
    # [ doc = "Bit 4 - Provides control of the DP PULLUP in the USB OTG module, if USB is configured in non-OTG device mode" ]
    pub fn dppullupnonotg(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ControlW {
    bits: u8,
}

impl ControlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ControlW { bits: 0 }
    }
    # [ doc = "Bit 4 - Provides control of the DP PULLUP in the USB OTG module, if USB is configured in non-OTG device mode" ]
    pub fn dppullupnonotg(&mut self, value: bool) -> &mut Self {
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
pub struct Usbtrc0 {
    register: ::volatile_register::RW<u8>,
}

impl Usbtrc0 {
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
        where for<'w> F: FnOnce(&Usbtrc0R, &'w mut Usbtrc0W) -> &'w mut Usbtrc0W
    {
        let bits = self.register.read();
        let r = Usbtrc0R { bits: bits };
        let mut w = Usbtrc0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Usbtrc0R {
        Usbtrc0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Usbtrc0W) -> &mut Usbtrc0W
    {
        let mut w = Usbtrc0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Usbtrc0R {
    bits: u8,
}

impl Usbtrc0R {
    # [ doc = "Bit 0 - USB Asynchronous Interrupt" ]
    pub fn usb_resume_int(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Synchronous USB Interrupt Detect" ]
    pub fn sync_det(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Asynchronous Resume Interrupt Enable" ]
    pub fn usbresmen(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Usbtrc0W {
    bits: u8,
}

impl Usbtrc0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Usbtrc0W { bits: 0 }
    }
    # [ doc = "Bit 5 - Asynchronous Resume Interrupt Enable" ]
    pub fn usbresmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - USB reset" ]
    pub fn usbreset(&mut self, value: bool) -> &mut Self {
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
pub struct Usbfrmadjust {
    register: ::volatile_register::RW<u8>,
}

impl Usbfrmadjust {
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
        where for<'w> F: FnOnce(&UsbfrmadjustR, &'w mut UsbfrmadjustW) -> &'w mut UsbfrmadjustW
    {
        let bits = self.register.read();
        let r = UsbfrmadjustR { bits: bits };
        let mut w = UsbfrmadjustW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> UsbfrmadjustR {
        UsbfrmadjustR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut UsbfrmadjustW) -> &mut UsbfrmadjustW
    {
        let mut w = UsbfrmadjustW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct UsbfrmadjustR {
    bits: u8,
}

impl UsbfrmadjustR {
    # [ doc = "Bits 0:7 - Frame Adjustment" ]
    pub fn adj(&self) -> u8 {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct UsbfrmadjustW {
    bits: u8,
}

impl UsbfrmadjustW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        UsbfrmadjustW { bits: 0 }
    }
    # [ doc = "Bits 0:7 - Frame Adjustment" ]
    pub fn adj(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u8) << OFFSET);
        self.bits |= ((value & MASK) as u8) << OFFSET;
        self
    }
}
