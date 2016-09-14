
// It is `read` and `write` because `in` is a rust keyword.

pub trait IOPorts {
    fn write(port: u16, data: Self);
    fn read(port: u16) -> Self;
}

pub fn write<T: IOPorts>(port: u16, data: T) {
    IOPorts::write(port, data);
}

pub fn read<T: IOPorts>(port: u16) -> T {
    IOPorts::read(port)
}

// implementations:

impl IOPorts for u8 {
    fn write(port: u16, data: u8) {
        unsafe { asm!("outb %al, %dx" :  : "{al}"(data), "{dx}"(port) :  : "volatile") };
    }
    fn read(port: u16) -> u8 {
        let ret: u8;
        unsafe { asm!("inb %dx, %al" : "={al}"(ret) : "{dx}"(port) :  : "volatile") };
        ret
    }
}

impl IOPorts for u16 {
    fn write(port: u16, data: u16) {
       unsafe { asm!("outw %ax, %dx" :  : "{ax}"(data), "{dx}"(port) :  : "volatile") };
    }
    fn read(port: u16) -> u16 {
        let ret: u16;
        unsafe { asm!("inw %dx, %ax" : "={ax}"(ret) : "{dx}"(port) :  : "volatile") };
        ret
    }
}

impl IOPorts for u32 {
    fn write(port: u16, data: u32) {
        unsafe { asm!("outl %eax, %dx" :  : "{eax}"(data), "{dx}"(port) :  : "volatile") };
    }
    fn read(port: u16) -> u32 {
        let ret: u32;
        unsafe { asm!("inl %dx, %eax" : "={eax}"(ret) : "{dx}"(port) :  : "volatile") };
        ret
    }
}
