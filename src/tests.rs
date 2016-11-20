
use ::prelude::*;

pub enum TestSetEntry {
    Test(&'static Fn() -> ()),
    Set(&'static TestSet),
}

impl TestSetEntry {
    pub fn run(&self) {
        match self {
            &Test(f) => {
                f()
            },
            &Set(s) => s.run(),
        }
    }
}

pub use self::TestSetEntry::Test;
pub use self::TestSetEntry::Set;

pub type TestsT = &'static [(TestSetEntry, &'static str)];

pub trait TestSet {
    const TESTS: TestsT;
}

//fn run_test_set<T: TestSet>() {
//    for entry in T::TESTS {
//        entry
//    }
//}

pub fn test_all() {


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

    struct SampleTestSet;

    fn foo() {
        assert!(0 == 0);
    }

    fn bar() {
        assert!(5 < 7);
    }

    impl TestSet for SampleTestSet {
        const TESTS: TestsT = &[
            (Test(&foo), "sample1"),
            (Test(&bar), "sample2"),
        ];
    }
}


