
prelude!();

/// Add your test set here
fn test_sets() {
    //run_test_set::<path_to_your_tests_module::Tests>();
    run_test_set::<::test_lib::sample_mod::sample_mod_tests::Tests>();
    run_test_set::<mem::paging::paging_tests::Tests>();
    run_test_set::<mem::buddy::buddy_tests::Tests>();
    run_test_set::<::ioports::ioports_tests::Tests>();
    run_test_set::<utility::utility_tests::Tests>();
//    run_test_set::<mem::slab::slab_tests::Tests>();
//    run_test_set::<mem::general_allocator::general_allocator_tests::Tests>();
}


// ======================================================================

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
    println!("");
    println!("Run all tests:");
    println!("");

    test_sets();

    println!("");
    println!("all tests passed [^_^]");
    println!("");
}

/// Sample of tests subcrate.
mod sample_mod {
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

