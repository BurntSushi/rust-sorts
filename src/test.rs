use quickcheck::quickcheck;
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
defsamelen!(samelen_merge, merge::sort)
defsamelen!(samelen_quicksort_dumb, quick::dumb)
defsamelen!(samelen_quicksort_smart, quick::smart)
defsamelen!(samelen_quicksort_simple,
            |xs: &mut [int]| as_mutable(xs, quick::simple))

defsorted!(sorted_std, std_sort)
defsorted!(sorted_insertion, super::insertion)
defsorted!(sorted_bubble, super::bubble)
defsorted!(sorted_merge, merge::sort)
defsorted!(sorted_quicksort_dumb, quick::dumb)
defsorted!(sorted_quicksort_smart, quick::smart)
defsorted!(sorted_quicksort_simple,
           |xs: &mut [int]| as_mutable(xs, quick::simple))

defstable!(stable_std, std_sort)
defstable!(stable_insertion, super::insertion)
defstable!(stable_bubble, super::bubble)
defstable!(stable_merge, merge::sort)
defstable!(stable_quicksort_simple,
           |xs: &mut [Pair]| as_mutable(xs, quick::simple))

defunstable!(unstable_quicksort_dumb, quick::dumb)
defunstable!(unstable_quicksort_smart, quick::smart)

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

fn as_mutable<T: TotalOrd + Clone>(xs: &mut [T], sort: fn(~[T]) -> ~[T]) {
    let ys = sort(xs.to_owned());
    let len = xs.len();
    xs.move_from(ys, 0, len);
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
