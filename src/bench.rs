#[allow(dead_code)];
#[allow(unused_imports)];

use rand::{Rng, task_rng};
use stdtest::BenchHarness;

use super::heap;
use super::merge;
use super::quick;

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
defbench!(sorted_bogo, bogo, sorted)
defbench!(sorted_insertion, insertion, sorted)
defbench!(sorted_bubble, bubble, sorted)
defbench!(sorted_selection, selection, sorted)
defbench!(sorted_mergesort, merge::sort, sorted)
defbench!(sorted_mergesort_insertion, merge::insertion, sorted)
defbench!(sorted_heapsort_up, heap::up, sorted)
defbench!(sorted_heapsort_down, heap::down, sorted)
defbench!(sorted_quicksort_dumb, quick::dumb, sorted)
defbench!(sorted_quicksort_smart, quick::smart, sorted)
defbench!(sorted_quicksort_insertion, quick::insertion, sorted)

defbench!(same_std, std_sort, same)
defbench!(same_bogo, bogo, same)
defbench!(same_insertion, insertion, same)
defbench!(same_bubble, bubble, same)
defbench!(same_selection, selection, same)
defbench!(same_mergesort, merge::sort, same)
defbench!(same_mergesort_insertion, merge::insertion, same)
defbench!(same_heapsort_up, heap::up, same)
defbench!(same_heapsort_down, heap::down, same)
defbench!(same_quicksort_dumb, quick::dumb, same)
defbench!(same_quicksort_smart, quick::smart, same)
defbench!(same_quicksort_insertion, quick::insertion, same)

defbench!(micro_std, std_sort, micro)
defbench!(micro_insertion, insertion, micro)
defbench!(micro_bubble, bubble, micro)
defbench!(micro_selection, selection, micro)
defbench!(micro_mergesort, merge::sort, micro)
defbench!(micro_mergesort_insertion, merge::insertion, micro)
defbench!(micro_heapsort_up, heap::up, micro)
defbench!(micro_heapsort_down, heap::down, micro)
defbench!(micro_quicksort_dumb, quick::dumb, micro)
defbench!(micro_quicksort_smart, quick::smart, micro)
defbench!(micro_quicksort_insertion, quick::insertion, micro)

defbench!(small_std, std_sort, small)
defbench!(small_insertion, insertion, small)
defbench!(small_bubble, bubble, small)
defbench!(small_selection, selection, small)
defbench!(small_mergesort, merge::sort, small)
defbench!(small_mergesort_insertion, merge::insertion, small)
defbench!(small_heapsort_up, heap::up, small)
defbench!(small_heapsort_down, heap::down, small)
defbench!(small_quicksort_dumb, quick::dumb, small)
defbench!(small_quicksort_smart, quick::smart, small)
defbench!(small_quicksort_insertion, quick::insertion, small)

// We stop benchmarking O(n^2) (and greater) sorting algorithms here. They
// are just too slow.

defbench!(medium_std, std_sort, medium)
defbench!(medium_mergesort, merge::sort, medium)
defbench!(medium_mergesort_insertion, merge::insertion, medium)
defbench!(medium_heapsort_up, heap::up, medium)
defbench!(medium_heapsort_down, heap::down, medium)
defbench!(medium_quicksort_dumb, quick::dumb, medium)
defbench!(medium_quicksort_smart, quick::smart, medium)
defbench!(medium_quicksort_insertion, quick::insertion, medium)

defbench!(large_std, std_sort, large)
defbench!(large_mergesort, merge::sort, large)
defbench!(large_mergesort_insertion, merge::insertion, large)
defbench!(large_heapsort_up, heap::up, large)
defbench!(large_heapsort_down, heap::down, large)
defbench!(large_quicksort_dumb, quick::dumb, large)
defbench!(large_quicksort_smart, quick::smart, large)
defbench!(large_quicksort_insertion, quick::insertion, large)

