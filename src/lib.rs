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

pub static INSERTION_THRESHOLD: uint = 16;

/// The `bogo` sort is the simplest but worst sorting algorithm. It shuffles
/// the given input until it is sorted. Its worst case space complexity is
/// `O(n)` but its time complexity is *unbounded*.
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

/// Classic in place insertion sort. Worst case time complexity is `O(n^2)`.
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

/// Classic in place bubble sort. Worst case time complexity is `O(n^2)`.
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

/// Classic in place selection sort. Worst case time complexity is `O(n^2)`.
/// Note that this is an *unstable* implementation.
pub fn selection<T: TotalOrd>(xs: &mut [T]) {
    let (mut i, len) = (0, xs.len());
    while i < len {
        let (mut j, mut cur_min) = (i + 1, i);
        while j < len {
            if xs[j] < xs[cur_min] {
                cur_min = j;
            }
            j = j + 1;
        }
        xs.swap(i, cur_min);
        i = i + 1;
    }
}

pub mod quick {
    use super::INSERTION_THRESHOLD;

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
        qsort(xs, smart_pivot)
    }

    pub fn insertion<T: TotalOrd>(xs: &mut [T]) {
        if xs.len() <= 1 {
            return
        }
        let p = smart_pivot(xs);
        let p = partition(xs, p);

        if p <= INSERTION_THRESHOLD {
            super::insertion(xs.mut_slice_to(p))
        } else {
            qsort(xs.mut_slice_to(p), smart_pivot);
        }
        if xs.len() - p+1 <= INSERTION_THRESHOLD {
            super::insertion(xs.mut_slice_from(p+1))
        } else {
            qsort(xs.mut_slice_from(p+1), smart_pivot);
        }
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

    fn smart_pivot<T: TotalOrd>(xs: &[T]) -> uint {
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
}

pub mod heap {
    pub fn up<T: TotalOrd>(xs: &mut [T]) {
        sort(xs, heapify_up);
    }

    pub fn down<T: TotalOrd>(xs: &mut [T]) {
        sort(xs, heapify_down);
    }

    fn sort<T: TotalOrd>(xs: &mut [T], heapify: fn(&mut [T])) {
        if xs.len() <= 1 {
            return
        }

        heapify(xs);
        let mut end = xs.len() - 1;
        while end > 0 {
            xs.swap(end, 0);
            end = end - 1;
            sift_down(xs, 0, end);
        }
    }

    fn heapify_down<T: TotalOrd>(xs: &mut [T]) {
        let last = xs.len() - 1;
        let mut start = 1 + ((last - 1) / 2);
        while start > 0 {
            start = start - 1;
            sift_down(xs, start, last);
        }
    }

    fn sift_down<T: TotalOrd>(xs: &mut [T], start: uint, end: uint) {
        let mut root = start;
        while root * 2 + 1 <= end {
            let child = root * 2 + 1;
            let mut swap = root;
            if xs[swap] < xs[child] {
                swap = child
            }
            if child + 1 <= end && xs[swap] < xs[child+1] {
                swap = child + 1
            }

            if swap == root {
                return
            }
            xs.swap(root, swap);
            root = swap;
        }
    }

    fn heapify_up<T: TotalOrd>(xs: &mut [T]) {
        let mut end = 1;
        while end < xs.len() {
            sift_up(xs, 0, end);
            end = end + 1;
        }
    }

    fn sift_up<T: TotalOrd>(xs: &mut [T], start: uint, end: uint) {
        let mut child = end;
        while child > start {
            let parent = (child - 1) / 2;
            if xs[parent] >= xs[child] {
                return
            }
            xs.swap(parent, child);
            child = parent;
        }
    }
}

pub mod merge {
    use std::cmp::min;
    use std::vec::MutableCloneableVector;

    use super::INSERTION_THRESHOLD;

    /// A stable mergesort with worst case `O(nlogn)` performance. This
    /// particular implementation has `O(n)` complexity. Unfortunately, the
    /// constant factor is pretty high.
    ///
    /// (See Rust's standard library `sort` function for a better mergesort
    /// which uses unsafe, I think.)
    pub fn sort<T: TotalOrd + Clone>(xs: &mut [T]) {
        let (len, mut width) = (xs.len(), 1);
        let mut buf = xs.to_owned();
        while width < len {
            let mut start = 0;
            while start < len {
                let mid = min(len, start + width);
                let end = min(len, start + 2 * width);
                merge(xs, buf, start, mid, end);
                start = start + 2 * width;
            }
            width = width * 2;
            xs.copy_from(buf);
        }
    }

    pub fn insertion<T: TotalOrd + Clone>(xs: &mut [T]) {
        let (len, mut width) = (xs.len(), INSERTION_THRESHOLD);
        let mut i = 0;
        while i < len {
            let upto = min(len, i + INSERTION_THRESHOLD);
            super::insertion(xs.mut_slice(i, upto));
            i = i + INSERTION_THRESHOLD;
        }

        let mut buf = xs.to_owned();
        while width < len {
            let mut start = 0;
            while start < len {
                let mid = min(len, start + width);
                let end = min(len, start + 2 * width);
                merge(xs, buf, start, mid, end);
                start = start + 2 * width;
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
