
macro_rules! test_set {
    ($path: path) => { run_test_set::<$path>() };
}

/// Add your tests subcrate here
pub fn test_all() {
    test_set!(::mem::buddy::buddy_tests::Tests);
    test_set!(::tests::sample_mod::sample_mod_tests::Tests);

    /*
    ::fmt::tests::all();
    ::ioports::ioports_tests::all();
    ::utility::utility_tests::all();
    ::mem::paging::tests::all();
    */
}


// ======================================================================

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


/// Sample of tests subcrate.
mod sample_mod {
    pub fn two() -> i32 { 2 }
    pub fn five() -> i32 { 5 }
    
    #[cfg(os_test)]
    pub mod sample_mod_tests {
        tests_subcrate!("sample_mod",
            sample1,
            two_less_than_five,
        );


        fn sample1() {
            assert!(0 == 0);
        }

        fn two_less_than_five() {
            assert!(two() < five());
        }
    }
}
