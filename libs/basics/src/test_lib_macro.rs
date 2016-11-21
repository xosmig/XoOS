
#[macro_export]
macro_rules! tests_module {
    ($name: expr, $( $test: ident ),* ) => {
        prelude!();
        pub struct Tests;
        impl TestSet for Tests {
            const NAME: test_lib::NameT = $name;
            const TESTS: test_lib::TestsT = &[
                $( (&$test, stringify!($test)), )*
            ];
        }
    };

    // version with extra comma at the end
    ($name: expr, $( $test: ident ),*, ) => {
        tests_module!($name $(, $test)*);
    }
}
