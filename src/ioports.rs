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

    unsafe fn read(&mut self) -> Self::Type {
        let ret: Self::Type;
        asm!("inb %dx, %al" : "={al}"(ret) : "{dx}"(self.num) :  : "volatile");
        ret
    }
}

impl<In> OutPort for IOPort<In, u8> {
    type Type = u8;

    unsafe fn write(&mut self, data: Self::Type) {
        asm!("outb %al, %dx" :  : "{al}"(data), "{dx}"(self.num) :  : "volatile");
    }
}

impl<Out> InPort for IOPort<u16, Out> {
    type Type = u16;

    unsafe fn read(&mut self) -> Self::Type {
        let ret: Self::Type;
        asm!("inw %dx, %ax" : "={ax}"(ret) : "{dx}"(self.num) :  : "volatile");
        ret
    }
}

impl<In> OutPort for IOPort<In, u16> {
    type Type = u16;

    unsafe fn write(&mut self, data: Self::Type) {
        asm!("outw %ax, %dx" :  : "{ax}"(data), "{dx}"(self.num) :  : "volatile");
    }
}

impl<Out> InPort for IOPort<u32, Out> {
    type Type = u32;

    unsafe fn read(&mut self) -> Self::Type {
        let ret: Self::Type;
        asm!("inl %dx, %eax" : "={eax}"(ret) : "{dx}"(self.num) :  : "volatile");
        ret
    }
}

impl<In> OutPort for IOPort<In, u32> {
    type Type = u32;

    unsafe fn write(&mut self, data: Self::Type) {
        asm!("outl %eax, %dx" :  : "{eax}"(data), "{dx}"(self.num) :  : "volatile");
    }
}
