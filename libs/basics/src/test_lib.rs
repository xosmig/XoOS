
prelude!();


pub type NameT = &'static str;
pub type TestsT = &'static [(&'static Fn() -> (), &'static str)];

pub trait TestSet {
    const NAME: NameT;
    const TESTS: TestsT;
}

pub fn run_test_set<T: TestSet>() {
    for &(test, test_name) in T::TESTS {
        print!("test \"{} :: {}\" ... ", T::NAME, test_name);
        test();
        println!("OK");
    }
}

/// Sample of tests subcrate.
pub mod sample_mod {
    pub fn two() -> i32 { 2 }
    pub fn five() -> i32 { 5 }

    #[cfg(os_test)]
    pub mod sample_mod_tests {
        use super::*;
        tests_module!("sample_mod",
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

