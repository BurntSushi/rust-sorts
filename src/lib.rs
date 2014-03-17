#[crate_id = "sorts#0.1.0"];
#[crate_type = "lib"];
#[license = "UNLICENSE"];
#[doc(html_root_url = "http://burntsushi.net/rustdoc/rust-sorts")];

#[feature(macro_rules)];
#[allow(deprecated_owned_vector)];

//! A collection of sorting algorithms with tests and benchmarks.

extern crate stdtest = "test";
extern crate quickcheck;
extern crate rand;

#[cfg(test)]
mod bench;

#[cfg(test)]
mod test;

pub fn insertion<T: TotalOrd>(xs: &mut [T]) {
    let mut i = 0;
    let mut j;
    let len = xs.len();
    while i < len {
        j = i+1;
        while j < len {
            if xs[i] > xs[j] {
                xs.swap(i, j)
            }
            j = j + 1;
        }
        i = i + 1;
    }
}

pub mod quicksort {
    /// A simple recursive quicksort that isn't afraid of allocation.
    /// Its average time complexity is `O(nlogn)` and its space complexity
    /// is also `O(nlogn)`.
    pub fn simple<T: TotalOrd + Clone>(xs: ~[T]) -> ~[T] {
        if xs.len() <= 1 {
            return xs
        }
        let pivot = xs[0].clone();
        let (mut left, mut right): (~[T], ~[T]) = (~[], ~[]);
        for x in xs.move_iter().skip(1) {
            if x <= pivot {
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
        let len = xs.len();
        if len <= 1 {
            return
        }
        partition(xs, false, 0, 0, len-1);
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
        let len = xs.len();
        if len <= 1 {
            return
        }
        partition(xs, true, 0, 0, len-1);
    }

    fn partition<T: TotalOrd>
                (xs: &mut [T], smart_pivot: bool, p: uint, l: uint, r: uint) {
        if (r-l+1) <= 1 {
            return
        }

        let mut nextp = l;
        xs.swap(r, p);
        let mut i = l;
        while i <= r-1 {
            if xs[i] <= xs[r] {
                xs.swap(i, nextp);
                nextp = nextp + 1;
            }
            i = i + 1;
        }
        xs.swap(nextp, r);

        let leftleft = if nextp == 0 { 0 } else { nextp-1 };
        let (mut lp, mut rp) = (l, nextp+1);
        if smart_pivot {
            lp = pivot_index(xs, l, leftleft);
            rp = pivot_index(xs, nextp+1, r);
        }
        partition(xs, smart_pivot, lp, l, leftleft);
        partition(xs, smart_pivot, rp, nextp+1, r);
    }

    // Returns the index of a pivot in `xs` given left `l` and right `r`
    // bounds. This particular approach uses the index of the median of the
    // values at the left, right and middle in `[l, r]`.
    fn pivot_index<T: TotalOrd>(xs: &mut [T], l: uint, r: uint) -> uint {
        if r <= l {
            return l
        }
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
