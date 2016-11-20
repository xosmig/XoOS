
use ::prelude::*;

pub type TestsT = &'static [(&'static Fn() -> (), &'static str)];

pub trait TestSet {
    const TESTS: TestsT;
}

fn run_test_set<T: TestSet>() {
    for &(test, name) in T::TESTS {
        print!("test \"{}\" ... ", name);
        test();
        println!("OK");
    }
}

pub fn test_all() {
    run_test_set::<::tests::sample_tests::SampleTestSet>()

    /*::fmt::tests::all();
    ::ioports::ioports_tests::all();
    ::utility::utility_tests::all();
    ::mem::paging::tests::all();
    ::mem::buddy::buddy_tests::all();*/
}


/// Sample of tests subcrate.
#[cfg(os_test)]
mod sample_tests {
    use ::tests::*;

    pub struct SampleTestSet;

    fn foo() {
        assert!(0 == 0);
    }

    fn bar() {
        assert!(5 < 7);
    }

    impl TestSet for SampleTestSet {
        const TESTS: TestsT = &[
            (&foo, "tests_lib sample1"),
            (&bar, "tests_lib sample2"),
        ];
    }
}


