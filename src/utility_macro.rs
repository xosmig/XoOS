

/// [val, val, val, ... (len times) ]
macro_rules! generate(
    ($val: expr; $len: expr) => (
        {
            let mut array: [_; $len] = unsafe { ::mem::uninitialized() };
            for i in array.iter_mut() {
                unsafe { ::core::ptr::write(i, $val); }
            }
            array
        }
    )
);

/// `try!` for `Option`
macro_rules! tryo(
    ($opt: expr) => (
        {
            match $opt {
                Some(obj) => obj,
                None => { return None; },
            }
        }
    )
);

/// A horrible macro to deceive the borrow checker
macro_rules! reborrow_mut {
    ($mref: expr) => ({ &mut *($mref as *mut _) });
    ($mref: expr, $t: tt) => ({ &mut *($mref as *mut $t) });
}

