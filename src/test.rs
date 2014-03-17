use quickcheck::quickcheck;

#[test]
fn std() {
    fn prop(mut xs: ~[int]) -> bool {
        xs.sort();
        is_sorted(xs)
    }
    quickcheck(prop);
}

#[test]
fn insertion() {
    fn prop(mut xs: ~[int]) -> bool {
        super::insertion(xs);
        is_sorted(xs)
    }
    quickcheck(prop);
}

#[test]
fn quicksort_simple() {
    fn prop(xs: ~[int]) -> bool {
        is_sorted(super::quicksort::simple(xs))
    }
    quickcheck(prop);
}

#[test]
fn quicksort_dumb() {
    fn prop(mut xs: ~[int]) -> bool {
        super::quicksort::dumb(xs);
        is_sorted(xs)
    }
    quickcheck(prop);
}

#[test]
fn quicksort_smart() {
    fn prop(mut xs: ~[int]) -> bool {
        super::quicksort::smart(xs);
        is_sorted(xs)
    }
    quickcheck(prop);
}

fn is_sorted<T: TotalOrd>(xs: &[T]) -> bool {
    for win in xs.windows(2) {
        if win[0] > win[1] {
            return false
        }
    }
    true
}
