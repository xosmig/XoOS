/// wrapper for `in` and `out` asm commands.
/// it is `read` and `write` because `in` is a rust keyword.

// FIXME: remake it! Create a `port` struct, which know what should be written and read.

use ::core::marker::PhantomData;

pub struct IOPort<In, Out> {
    num: u16,
    phantom_in: PhantomData<In>,
    phantom_out: PhantomData<Out>,
}

impl<In, Out> IOPort<In, Out> {
    pub const fn new(num: u16) -> Self {
        IOPort { num: num, phantom_in: PhantomData{}, phantom_out: PhantomData{} }
    }
}

pub trait InPort {
    type Type;
    unsafe fn read(&mut self) -> Self::Type;
}

pub trait OutPort {
    type Type;
    unsafe fn write(&mut self, data: Self::Type);
}

impl<Out> InPort for IOPort<u8, Out> {
    type Type = u8;

    unsafe fn read(&mut self) -> u8 {
        let ret: u8;
        asm!("inb %dx, %al" : "={al}"(ret) : "{dx}"(self.num) :  : "volatile");
        ret
    }
}

impl<In> OutPort for IOPort<In, u8> {
    type Type = u8;

    unsafe fn write(&mut self, data: u8) {
        asm!("outb %al, %dx" :  : "{al}"(data), "{dx}"(self.num) :  : "volatile");
    }
}
