
use ::prelude::*;

pub type NameT = &'static str;
pub type TestsT = &'static [(&'static Fn() -> (), &'static str)];

pub trait TestSet {
    const NAME: NameT;
    const TESTS: TestsT;
}

fn run_test_set<T: TestSet>() {
    for &(test, test_name) in T::TESTS {
        print!("test \"{} :: {}\" ... ", T::NAME, test_name);
        test();
        println!("OK");
    }
}

pub fn test_all() {
    run_test_set::<::tests::sample_mod::sample_mod_tests::Tests>()

    /*::fmt::tests::all();
    ::ioports::ioports_tests::all();
    ::utility::utility_tests::all();
    ::mem::paging::tests::all();
    ::mem::buddy::buddy_tests::all();*/
}

macro_rules! tests_subcrate {
    ($name: expr, $tests: expr) => {
        use super::*;
        use ::prelude::*;
        use ::tests::*;
        pub struct Tests;

        impl TestSet for Tests {
            const NAME: NameT = $name;
            const TESTS: TestsT = &$tests;
        }
    };
}


/// Sample of tests subcrate.
mod sample_mod {
    pub fn two() -> i32 { 2 }
    pub fn five() -> i32 { 5 }
    
    #[cfg(os_test)]
    pub mod sample_mod_tests {
        tests_subcrate!("sample_mod", [
            (&foo, "sample1"),
            (&two_less_than_five, "sample2"),
        ]);

        fn foo() {
            assert!(0 == 0);
        }

        fn two_less_than_five() {
            assert!(two() < five());
        }
    }
}
