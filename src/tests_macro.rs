
macro_rules! tests_module {
    ($name: expr, $( $test: ident ),*, ) => {
        use super::*;
        use ::prelude::*;
        use ::tests::*;
        pub struct Tests;

        impl TestSet for Tests {
            const NAME: NameT = $name;
            const TESTS: TestsT = &[
                $( (&$test, stringify!($test)), )*
            ];
        }
    };
}