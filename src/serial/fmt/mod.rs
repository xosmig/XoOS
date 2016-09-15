
// FIXME: It all should be in another module (not connected with serial port)
// FIXME: And must be able to write in different places (for example: in char buffer)
// FIXME: i.e. implement trait similar to `core::fmt::Write`
// FIXME: or fix &str arguments and use `core::fmt::Write`
// FIXME: but don't delete this code!! It's really cool :)

mod with_macro;

pub use self::with_macro::*;

macro_rules! print {
    ( $( $x:expr ),* ) => {
        {
            $(
                $x.print();
            )*
        }
    };
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn run_all() { // FIXME: how to use it via make?
        integration1();
    }

    fn integration1() {
        // this test should be checked manually.
        // correct output is:
        // This is [12, -154, 0] in octal: [14, -232, 0] and in hex: [c, -9a, 0].

        let x = [12, -154, 0];
        print!("This is ", x, " in octal: ", octal(&x), " and in hex: ", hex(&x), ".\n");
    }
}
