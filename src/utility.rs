
// unfortunately, there is no such a trait in the standard library
// extern crates use `std`
pub trait UInt: Copy {
    fn from64(x: u64) -> Self;
    fn to64(self) -> u64;
}

macro_rules! implement {
    ($t: ty) => (
        impl UInt for $t {
            fn from64(x: u64) -> $t {
                x as $t
            }
            fn to64(self) -> u64 {
                self as u64
            }
        }
    );
}

implement!(u8);
implement!(u16);
implement!(u32);
implement!(u64);
implement!(usize);

pub fn bit<R: UInt>(num: u8) -> R {
    R::from64((1 as u64) << num)
}

pub fn get_bit<T: UInt>(x: T, num: u8) -> bool {
    (x.to64() & (1 << num)) != 0
}

pub fn log2_floor<T: UInt>(x: T) -> usize {
    64 - (x.to64().leading_zeros() as usize) - 1
}

pub fn log2_ceil<T: UInt>(x: T) -> usize {
    let mut ret = log2_floor(x);
    if x.to64() > (1 << ret) {
        ret += 1
    }
    ret
}

pub fn dist<T>(begin: *const T, end: *const T) -> isize {
    (end as isize) - (begin as isize)
}

pub fn round_up<T: UInt>(x: T, base: T) -> T {
    let x = x.to64();
    let base = base.to64();
    let r = x % base;
    T::from64(if r == 0 { x } else { x - r + base })
}

pub fn round_down<T:UInt>(x: T, base:T) -> T {
    let base = base.to64();
    T::from64((x.to64() / base) * base)
}


#[cfg(os_test)]
pub mod utility_tests {
    use super::*;
    tests_module!("utility",
        log2_floor_test,
        log2_ceil_test,
    );


    fn log2_floor_test() {
        assert_eq!(0, log2_floor(1 as u8));
        assert_eq!(1, log2_floor(2 as u16));
        assert_eq!(1, log2_floor(3 as u32));
        assert_eq!(2, log2_floor(4 as u64));
        assert_eq!(8, log2_floor(257 as u64));
    }

    fn log2_ceil_test() {
        assert_eq!(0, log2_ceil(1 as u8));
        assert_eq!(1, log2_ceil(2 as u16));
        assert_eq!(2, log2_ceil(3 as u32));
        assert_eq!(2, log2_ceil(4 as u64));
        assert_eq!(9, log2_ceil(257 as u64));
    }
}
