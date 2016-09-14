
use core::ops::Shl;

pub fn bit<T>(num: u8) -> T where T: Shl<T, Output=T> + From<u8> {
    T::from(1) << T::from(num)
}
