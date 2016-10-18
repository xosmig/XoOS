
use ::core::convert::TryFrom;

// unfortunately, there is no such a trait in the standard library
// extern crates use `std`
pub trait UInt: Copy + Into<u64> + TryFrom<u64> {
    fn from(x: u64) -> Self {
        TryFrom::try_from(x).ok().unwrap()
    }
    fn get(self) -> u64 {
        self.into()
    }
}

impl UInt for u8 {}
impl UInt for u16 {}
impl UInt for u32 {}
impl UInt for u64 {}

pub fn bit<R: UInt>(num: u8) -> R {
    R::from(1 << num.get())
}

pub fn get_bit<T: UInt>(x: T, num: u8) -> bool {
    (x.get() & (1 << num)) != 0
}
