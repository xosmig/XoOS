
macro_rules! generate_by_cast {
    (Trait: $Trait: ty; Fn: $Fn: ident; Target: $Target: ty; Items: $( $Item: ty ),*) => (
        $(
            impl $Trait for $Item {
                fn $Fn(&self) {
                    (*self as $Target).$Fn();
                }
            }
        )*
    );
}

macro_rules! generate_for_arrays {
    (Trait: $Trait: ident; Fn: $Fn: ident; Sizes: $( $Size: expr ),* ) => (
        $(
            impl<T: $Trait> $Trait for [T; $Size] {
                fn $Fn(&self) {
                    Serial::get().write_byte(b'[');
                    let mut iter = self.into_iter();
                    iter.next().unwrap().$Fn();
                    for x in iter {
                        Serial::get().write_str(b", ");
                        x.$Fn();
                    }
                    Serial::get().write_byte(b']');
                }
            }
        )*
    );
}
