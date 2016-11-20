
macro_rules! tests_module {
    ($name: expr, $( $test: ident ),* ) => {
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

    // version with extra comma at the end
    ($name: expr, $( $test: ident ),*, ) => {
        tests_module!($name $(, $test)*);
    }
}
