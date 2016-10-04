
pub fn bit(num: u8) -> u8 {
    1 << num
}

// strange approach :/
macro_rules! concat_id {
    ( $( $id: ident ),* ) => (
        expand_string_to_expr!("fn ", concat!($( stringify!($id) ),* ))
    );
}

