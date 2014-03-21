use quickcheck::quickcheck;
use super::heap;
use super::merge;
use super::quick;

macro_rules! defsamelen(
    ($name:ident, $sorter:expr) => (
        #[test]
        fn $name() {
            fn prop(mut xs: ~[int]) -> bool {
                let len = xs.len();
                $sorter(xs);
                len == xs.len()
            }
            quickcheck(prop);
        }
    );
)

macro_rules! defsorted(
    ($name:ident, $sorter:expr) => (
        #[test]
        fn $name() {
            fn prop(mut xs: ~[int]) -> bool {
                $sorter(xs);
                is_sorted(xs)
            }
            quickcheck(prop);
        }
    );
)

macro_rules! defstable(
    ($name:ident, $sorter:expr) => (
        #[test]
        fn $name() {
            fn prop(xs: ~[int]) -> bool {
                let mut sxs = stable_annotate(xs);
                $sorter(sxs);
                is_stable(sxs)
            }
            quickcheck(prop);
        }
    );
)

macro_rules! defunstable(
    ($name:ident, $sorter:expr) => (
        #[test]
        #[should_fail]
        fn $name() {
            fn prop(xs: ~[int]) -> bool {
                let mut sxs = stable_annotate(xs);
                $sorter(sxs);
                is_stable(sxs)
            }
            quickcheck(prop);
        }
    );
)

defsamelen!(samelen_std, std_sort)
defsamelen!(samelen_insertion, super::insertion)
defsamelen!(samelen_bubble, super::bubble)
defsamelen!(samelen_selection, super::selection)
defsamelen!(samelen_merge, merge::sort)
defsamelen!(samelen_merge_insertion, merge::insertion)
defsamelen!(samelen_heapsort_up, heap::up)
defsamelen!(samelen_heapsort_down, heap::down)
defsamelen!(samelen_quicksort_dumb, quick::dumb)
defsamelen!(samelen_quicksort_smart, quick::smart)
defsamelen!(samelen_quicksort_insertion, quick::insertion)

defsorted!(sorted_std, std_sort)
defsorted!(sorted_insertion, super::insertion)
defsorted!(sorted_bubble, super::bubble)
defsorted!(sorted_selection, super::selection)
defsorted!(sorted_merge, merge::sort)
defsorted!(sorted_merge_insertion, merge::insertion)
defsorted!(sorted_heapsort_up, heap::up)
defsorted!(sorted_heapsort_down, heap::down)
defsorted!(sorted_quicksort_dumb, quick::dumb)
defsorted!(sorted_quicksort_smart, quick::smart)
defsorted!(sorted_quicksort_insertion, quick::insertion)

defstable!(stable_std, std_sort)
defstable!(stable_insertion, super::insertion)
defstable!(stable_bubble, super::bubble)
defstable!(stable_merge, merge::sort)
defstable!(stable_merge_insertion, merge::insertion)

defunstable!(unstable_selection, super::selection)
defunstable!(unstable_heapsort_up, heap::up)
defunstable!(unstable_heapsort_down, heap::down)
defunstable!(unstable_quicksort_dumb, quick::dumb)
defunstable!(unstable_quicksort_smart, quick::smart)
defunstable!(unstable_quicksort_insertion, quick::insertion)

fn std_sort<T: TotalOrd>(xs: &mut [T]) {
    xs.sort()
}

fn is_sorted<T: TotalOrd>(xs: &[T]) -> bool {
    for win in xs.windows(2) {
        if win[0] > win[1] {
            return false
        }
    }
    true
}

#[deriving(Show, Clone)]
struct Pair {
    val: int,
    vestigial: uint,
}

impl Eq for Pair {
    fn eq(&self, other: &Pair) -> bool {
        self.val == other.val
    }
}

impl Ord for Pair {
    fn lt(&self, other: &Pair) -> bool {
        self.val < other.val
    }
}

impl TotalEq for Pair {
    fn equals(&self, other: &Pair) -> bool {
        self.val == other.val
    }
}

impl TotalOrd for Pair {
    fn cmp(&self, other: &Pair) -> Ordering {
        self.val.cmp(&other.val)
    }
}

fn stable_annotate(xs: &[int]) -> ~[Pair] {
    let mut pairs = ~[];
    for (i, &x) in xs.iter().enumerate() {
        pairs.push(Pair { val: x, vestigial: i, })
    }
    pairs
}

fn is_stable(xs: &[Pair]) -> bool {
    fn vestigial_groups(xs: &[Pair]) -> ~[~[uint]] {
        let mut groups: ~[~[uint]] = ~[];
        let mut current: ~[uint] = ~[];
        for (i, &x) in xs.iter().enumerate() {
            if i == 0 {
                current.push(x.vestigial);
                continue
            }
            if xs[i-1].val == x.val {
                current.push(x.vestigial)
            } else {
                groups.push(current);
                current = ~[x.vestigial];
            }
        }
        if current.len() > 0 {
            groups.push(current)
        }
        groups
    }
    let groups = vestigial_groups(xs);
    // debug!("BEFORE GROUPS: {}", xs); 
    // debug!("GROUPS: {}", groups); 
    groups.move_iter().all(|g| is_sorted(g))
}
