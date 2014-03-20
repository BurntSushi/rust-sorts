#[allow(dead_code)];
#[allow(unused_imports)];

use rand::{Rng, task_rng};
use stdtest::BenchHarness;

static SIZE_SORTED: uint = 1000;
static SIZE_SAME: uint = 1000;
static SIZE_MICRO: uint = 10;
static SIZE_SMALL: uint = 100;
static SIZE_MEDIUM: uint = 10000;
static SIZE_LARGE: uint = 100000;

macro_rules! defbench(
    ($name:ident, $sorter:expr, $gen:ident) => (
        #[bench]
        #[cfg($gen)]
        fn $name(b: &mut BenchHarness) {
            let xs = $gen();
            b.iter(|| $sorter(xs.clone()))
        }
    );
)

fn std_sort(xs: &mut [int]) { xs.sort() }
fn micro() -> ~[int] { task_rng().gen_vec(SIZE_MICRO) }
fn small() -> ~[int] { task_rng().gen_vec(SIZE_SMALL) }
fn medium() -> ~[int] { task_rng().gen_vec(SIZE_MEDIUM) }
fn large() -> ~[int] { task_rng().gen_vec(SIZE_LARGE) }
fn sorted() -> ~[int] {
    let mut xs = task_rng().gen_vec(SIZE_SORTED);
    xs.sort();
    return xs
}
fn same() -> ~[int] {
    let x: int = task_rng().gen();
    ::std::iter::Repeat::new(x).take(SIZE_SAME).collect()
}

defbench!(sorted_std, std_sort, sorted)
defbench!(sorted_bogo, super::bogo, sorted)
defbench!(sorted_insertion, super::insertion, sorted)
defbench!(sorted_bubble, super::bubble, sorted)
defbench!(sorted_mergesort, super::merge::sort, sorted)
defbench!(sorted_quicksort_simple, super::quick::simple, sorted)
defbench!(sorted_quicksort_dumb, super::quick::dumb, sorted)
defbench!(sorted_quicksort_smart, super::quick::smart, sorted)

defbench!(same_std, std_sort, same)
defbench!(same_bogo, super::bogo, same)
defbench!(same_insertion, super::insertion, same)
defbench!(same_bubble, super::bubble, same)
defbench!(same_mergesort, super::merge::sort, same)
defbench!(same_quicksort_simple, super::quick::simple, same)
defbench!(same_quicksort_dumb, super::quick::dumb, same)
defbench!(same_quicksort_smart, super::quick::smart, same)

defbench!(micro_std, std_sort, micro)
defbench!(micro_insertion, super::insertion, micro)
defbench!(micro_bubble, super::bubble, micro)
defbench!(micro_mergesort, super::merge::sort, micro)
defbench!(micro_quicksort_simple, super::quick::simple, micro)
defbench!(micro_quicksort_dumb, super::quick::dumb, micro)
defbench!(micro_quicksort_smart, super::quick::smart, micro)

defbench!(small_std, std_sort, small)
defbench!(small_insertion, super::insertion, small)
defbench!(small_bubble, super::bubble, small)
defbench!(small_mergesort, super::merge::sort, small)
defbench!(small_quicksort_simple, super::quick::simple, small)
defbench!(small_quicksort_dumb, super::quick::dumb, small)
defbench!(small_quicksort_smart, super::quick::smart, small)

// We stop bench insertion sort here. Really slow.

defbench!(medium_std, std_sort, medium)
defbench!(medium_mergesort, super::merge::sort, medium)
defbench!(medium_quicksort_simple, super::quick::simple, medium)
defbench!(medium_quicksort_dumb, super::quick::dumb, medium)
defbench!(medium_quicksort_smart, super::quick::smart, medium)

defbench!(large_std, std_sort, large)
defbench!(large_mergesort, super::merge::sort, large)
defbench!(large_quicksort_simple, super::quick::simple, large)
defbench!(large_quicksort_dumb, super::quick::dumb, large)
defbench!(large_quicksort_smart, super::quick::smart, large)

