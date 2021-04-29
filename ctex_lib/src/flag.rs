#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Version {
    Legacy = 0x00,
    V02 = 0x01,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Compression {
    None = 0x00,
    Lz4 = 0x01,
}

#[repr(packed)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Flags {
    pub(crate) vers_0: Version,
    pub(crate) comp_0: Compression,
    pub(crate) lutw_0: u8,
    pub(crate) offw_0: u16,
    pub(crate) resv_0: [u8; 3],
}

impl Flags {
    pub fn default() -> Self {
        Flags {
            vers_0: Version::V02,
            comp_0: Compression::Lz4,
            lutw_0: 0x00,
            offw_0: 0x00_00,
            resv_0: [0x00, 0x00, 0x00],
        }
    }

    pub fn default_no_compression() -> Self {
        Flags {
            vers_0: Version::V02,
            comp_0: Compression::None,
            lutw_0: 0x00,
            offw_0: 0x00_00,
            resv_0: [0x00, 0x00, 0x00],
        }
    }

    pub fn as_u64(&self) -> u64 {
        unsafe { std::mem::transmute::<Self, u64>(*self) }
    }

    pub fn compression(&self) -> Compression {
        self.comp_0
    }

    pub fn version(&self) -> Version {
        self.vers_0
    }

    pub fn lut_len(&self) -> usize {
        self.lutw_0 as usize
    }

    pub fn offsets_len(&self) -> usize {
        self.offw_0 as usize * self.offw_0 as usize
    }

    pub fn offsets_width(&self) -> usize {
        self.offw_0 as usize
    }
}

impl From<u64> for Flags {
    fn from(v: u64) -> Self {
        unsafe { std::mem::transmute::<u64, Self>(v) }
    }
}
