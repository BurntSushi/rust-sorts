#[crate_id = "sorts#0.1.0"];
#[crate_type = "lib"];
#[license = "UNLICENSE"];
#[doc(html_root_url = "http://burntsushi.net/rustdoc/rust-sorts")];

#[feature(phase)];
#[feature(macro_rules)];
#[allow(deprecated_owned_vector)];

//! A collection of sorting algorithms with tests and benchmarks.

#[phase(syntax, link)] extern crate log;
extern crate stdtest = "test";
extern crate quickcheck;
extern crate rand;

use rand::Rng; // why do I need this?

#[cfg(test)]
mod bench;

#[cfg(test)]
mod test;

pub fn bogo<T: TotalOrd>(xs: &mut [T]) {
    fn is_sorted<T: TotalOrd>(xs: &[T]) -> bool {
        for win in xs.windows(2) {
            if win[0] > win[1] {
                return false
            }
        }
        true
    }
    let rng = &mut rand::task_rng();
    while !is_sorted(xs) {
        rng.shuffle_mut(xs);
    }
}

pub fn insertion<T: TotalOrd>(xs: &mut [T]) {
    let (mut i, len) = (1, xs.len());
    while i < len {
        let mut j = i;
        while j > 0 && xs[j-1] > xs[j] {
            xs.swap(j, j-1);
            j = j - 1;
        }
        i = i + 1;
    }
}

pub fn bubble<T: TotalOrd>(xs: &mut [T]) {
    let mut n = xs.len();
    while n > 0 {
        let mut newn = 0;
        let mut i = 1;
        while i < n {
            if xs[i-1] > xs[i] {
                xs.swap(i-1, i);
                newn = i;
            }
            i = i + 1;
        }
        n = newn;
    }
}

pub mod quick {
    /// A simple recursive quicksort that isn't afraid of allocation.
    /// Its average time complexity is `O(nlogn)` and its space complexity
    /// is also `O(nlogn)`.
    pub fn simple<T: TotalOrd>(mut xs: ~[T]) -> ~[T] {
        if xs.len() <= 1 {
            return xs
        }
        let pivot = xs.shift().unwrap();
        let (mut left, mut right): (~[T], ~[T]) = (~[], ~[]);
        for x in xs.move_iter() {
            if x < pivot {
                left.push(x)
            } else {
                right.push(x)
            }
        }

        let mut left = simple(left);
        let right = simple(right);
        left.push(pivot);
        for x in right.move_iter() {
            left.push(x)
        }
        left
    }

    /// Standard in-place quicksort that always uses the first element as
    /// a pivot. Average time complexity is `O(nlogn)` and its space complexity
    /// is `O(1)` (limited to vectors of size `N`, which is the maximum number
    /// expressible with a `uint`).
    pub fn dumb<T: TotalOrd>(xs: &mut [T]) {
        fn pivot<T: TotalOrd>(_: &[T]) -> uint { 0 }
        qsort(xs, pivot)
    }


    /// Standard in-place quicksort that uses the median of the first, middle
    /// and last elements in each vector for the pivot.
    /// Average time complexity is `O(nlogn)` and its space complexity
    /// is `O(1)` (limited to vectors of size `N`, which is the maximum number
    /// expressible with a `uint`).
    ///
    /// This seems to have the same performance characteristics as the `dumb`
    /// quicksort, except when the input is almost sorted where intelligently
    /// choosing a pivot helps by at least an order of magnitude. (This is
    /// because an almost-sorted vector given to the `dumb` quicksort provokes
    /// worse case `O(n^2)` performance, whereas picking a pivot intelligently
    /// helps keep it closer to the average `O(nlogn)` performance.)
    pub fn smart<T: TotalOrd>(xs: &mut [T]) {
        fn pivot<T: TotalOrd>(xs: &[T]) -> uint {
            let (l, r) = (0, xs.len() - 1);
            let m = l + ((r - l) / 2);
            let (left, middle, right) = (&xs[l], &xs[m], &xs[r]);
            if middle >= left && middle <= right {
                m
            } else if left >= middle && left <= right {
                l
            } else {
                r
            }
        }
        qsort(xs, pivot)
    }

    fn qsort<T: TotalOrd>(xs: &mut [T], pivot: fn(&[T]) -> uint) {
        if xs.len() <= 1 {
            return
        }
        let p = pivot(xs);
        let p = partition(xs, p);
        qsort(xs.mut_slice_to(p), pivot);
        qsort(xs.mut_slice_from(p+1), pivot);
    }

    fn partition<T: TotalOrd>(xs: &mut [T], p: uint) -> uint {
        if xs.len() <= 1 {
            return p
        }

        let lasti = xs.len() - 1;
        let (mut i, mut nextp) = (0, 0);
        xs.swap(lasti, p);
        while i < lasti {
            if xs[i] <= xs[lasti] {
                xs.swap(i, nextp);
                nextp = nextp + 1;
            }
            i = i + 1;
        }
        xs.swap(nextp, lasti);
        nextp
    }
}

pub mod merge {
    use std::cmp::min;
    use std::fmt::Show;
    use std::vec;
    use std::vec::MutableCloneableVector;

    pub fn sort<T: TotalOrd + Clone + Show>(xs: &mut [T]) {
        let (len, mut width) = (xs.len(), 1);
        let mut buf = vec::append(~[], xs);
        while width < len {
            let mut i = 0;
            while i < len {
                merge(xs, buf, i, min(len, i + width), min(len, i + 2*width));
                i = i + 2 * width;
            }
            width = width * 2;
            xs.copy_from(buf);
        }
    }

    fn merge<T: TotalOrd + Clone>
            (xs: &mut [T], buf: &mut [T], l: uint, r: uint, e: uint) {
        let (mut il, mut ir) = (l, r);
        let mut i = l;
        while i < e {
            if il < r && (ir >= e || xs[il] <= xs[ir]) {
                buf[i] = xs[il].clone();
                il = il + 1;
            } else {
                buf[i] = xs[ir].clone();
                ir = ir + 1;
            }
            i = i + 1;
        }
    }
}
