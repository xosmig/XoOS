
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
        let msg1 = "This is "; // FIXME
        let msg2 = " in octal: ";
        let msg3 = " and in hex: ";
        let msg4 = ".\n";
        print!(msg1, x, msg2, octal(&x), msg3, hex(&x), msg4);
    }
}
