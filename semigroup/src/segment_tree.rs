use std::ops::{Bound, Range, RangeBounds};

use crate::{Monoid, Semigroup};

pub mod index;
pub mod iter;

/// [`SegmentTree`] is a data structure for efficient range queries based on perfect binary tree.
/// It requires the underlying operation on the data to form a [`Monoid`].
///
/// # Examples
/// ```
/// use semigroup::{op::Sum, segment_tree::SegmentTree};
/// let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let mut sum_tree: SegmentTree<_> = data.into_iter().map(Sum).collect();
/// assert_eq!(sum_tree.combine(3..=5).0, 12);
/// sum_tree.update(4, 8.into());
/// assert_eq!(sum_tree.combine(3..=5).0, 16);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct SegmentTree<T> {
    tree: Vec<T>, // 1-indexed perfect binary tree, left child: 2i, right child: 2i+1, parent: i/2
    len: usize,
}
impl<T: Monoid + Clone> FromIterator<T> for SegmentTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iterator = iter.into_iter();
        let (lower, upper) = iterator.size_hint();
        match upper.filter(|&u| u == lower) {
            Some(len) => Self::new().construct(len, iterator),
            None => Self::from(iterator.collect::<Vec<_>>()),
        }
    }
}
impl<T: Monoid + Clone> From<Vec<T>> for SegmentTree<T> {
    fn from(v: Vec<T>) -> Self {
        Self::new().construct(v.len(), v)
    }
}
impl<T> SegmentTree<T> {
    /// **O(1)**, init empty segment tree.
    pub fn new() -> Self {
        let (tree, len) = (Vec::new(), 0);
        Self { tree, len }
    }
    /// **O(1)**, get size of the segment tree by given length.
    #[inline]
    fn capacity(len: usize) -> usize {
        2 * len.next_power_of_two()
    }
    /// **O(1)**, check if this segment tree can be extended by given length.
    fn over_capacity(&self, len: usize) -> bool {
        Self::capacity(self.len()) < Self::capacity(len)
    }
    /// **O(1)**, get beginning index of the segment tree leaf.
    #[inline]
    fn leaf_offset(&self) -> usize {
        self.len().next_power_of_two()
    }
    /// **O(1)**, return this segment tree's number of data.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }
    /// **O(1)**, check if this segment tree is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<T: Monoid + Clone> SegmentTree<T> {
    /// **O(n)**, construct segment tree by given data.
    fn construct<I: IntoIterator<Item = T>>(mut self, len: usize, iter: I) -> Self {
        self.resize_upto(len);
        self.reconstruct(iter);
        self
    }
    /// **O(len)**, resize segment tree by allocating identity elements without truncating.
    fn resize_upto(&mut self, len: usize) {
        let data = self.over_capacity(len).then(|| self[..].to_vec());
        self.tree.resize_with(Self::capacity(len), Monoid::identity);
        self.len = len.max(self.len());
        data.into_iter().for_each(|d| self.reconstruct(d));
    }
    /// **O(n)**, reconstruct segment tree by given data.
    fn reconstruct<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let leaf_offset = self.leaf_offset();
        for (i, d) in iter.into_iter().enumerate() {
            self.tree[leaf_offset + i] = d;
        }
        for i in (1..leaf_offset).rev() {
            self.tree[i] = Semigroup::op(self.tree[i * 2].clone(), self.tree[i * 2 + 1].clone());
        }
    }

    /// **O(log(n))**, set `leaf[i] = x`, and update segment tree.
    pub fn update(&mut self, i: usize, x: T) -> Option<T> {
        self.update_with(i, |_| x)
    }
    /// **O(log(n))**, update `leaf[i]` by `f(leaf[i])`, and update segment tree.
    pub fn update_with<F>(&mut self, i: usize, f: F) -> Option<T>
    where
        F: FnOnce(&T) -> T,
    {
        (i < self.len()).then(|| {
            let mut node = self.leaf_offset() + i;
            let mut result = f(&self.tree[node]);
            std::mem::swap(&mut self.tree[node], &mut result);
            while node > 1 {
                node /= 2;
                self.tree[node] =
                    Semigroup::op(self.tree[node * 2].clone(), self.tree[node * 2 + 1].clone());
            }
            result
        })
    }
    /// amortized **O(log(n))**, push `x` to the segment tree, when construct new segment tree should be used [`Self::from`] or [`Self::from_iter`] ([`std::iter::Iterator::collect`]) instead.
    pub fn push(&mut self, x: T) {
        self.resize_upto(self.len() + 1);
        self.update(self.len() - 1, x);
    }
    /// amortized **O(len(slice) log(n))**, extend segment tree by given slice.
    pub fn extend_from_slice(&mut self, slice: &[T]) {
        self.extend_with_length(slice.len(), slice.iter().cloned());
    }
    /// amortized **O(len log(n))**, extend segment tree by given iterator with given length.
    fn extend_with_length<I: IntoIterator<Item = T>>(&mut self, len: usize, iter: I) {
        if self.over_capacity(len) {
            let data: Vec<_> = self[..].iter().cloned().chain(iter).collect();
            self.resize_upto(self.len() + len);
            self.reconstruct(data);
        } else {
            let repeat_identity = std::iter::repeat_with(Monoid::identity);
            for d in iter.into_iter().chain(repeat_identity).take(len) {
                self.push(d.clone());
            }
        }
    }

    /// **O(1)**, get half-open interval range of the segment tree leaf.
    fn indices<R>(&self, range: R) -> Range<usize>
    where
        R: RangeBounds<usize>,
    {
        // TODO `std::slice::range` is nightly only https://doc.rust-lang.org/std/slice/fn.range.html
        let start = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Excluded(&l) => (l + 1).max(0),
            Bound::Included(&l) => l.max(0),
        };
        let end = match range.end_bound() {
            Bound::Unbounded => self.len(),
            Bound::Excluded(&r) => r.min(self.len()),
            Bound::Included(&r) => (r + 1).min(self.len()),
        };
        start..end
    }

    /// **O(log(n))**, combine the range.
    pub fn combine<R>(&self, range: R) -> T
    where
        R: RangeBounds<usize>,
    {
        let Range { start, end } = self.indices(range);
        let (mut left, mut right) = (self.leaf_offset() + start, self.leaf_offset() + end);
        let mut res = Monoid::identity();
        while left < right {
            if left % 2 == 1 {
                res = Semigroup::op(res, self.tree[left].clone());
                left += 1;
            }
            if right % 2 == 1 {
                right -= 1;
                res = Semigroup::op(self.tree[right].clone(), res);
            }
            left /= 2;
            right /= 2;
        }
        res
    }

    /// **O(log^2(n))**, search the leftmost leaf where `cmp(x)` is true in the range.
    pub fn bisect_left<R, F>(&self, range: R, cmp: F) -> Option<usize>
    where
        R: RangeBounds<usize>,
        F: Fn(&T) -> bool,
    {
        let Range { mut start, mut end } = self.indices(range);
        while end - start > 1 {
            let mid = (start + end) / 2;
            if cmp(&self.combine(start..mid)) {
                end = mid;
            } else {
                start = mid;
            }
        }
        cmp(&self.tree[self.leaf_offset() + start]).then_some(start)
    }
    /// **O(log^2(n))**, search the rightmost leaf where `cmp(x)` is true in the range.
    pub fn bisect_right<R, F>(&self, range: R, cmp: F) -> Option<usize>
    where
        R: RangeBounds<usize>,
        F: Fn(&T) -> bool,
    {
        let Range { mut start, mut end } = self.indices(range);
        while end - start > 1 {
            let mid = (start + end) / 2;
            if cmp(&self.combine(mid..end)) {
                start = mid;
            } else {
                end = mid;
            }
        }
        cmp(&self.tree[self.leaf_offset() + start]).then_some(start)
    }
}
impl<T: Monoid + Clone> Extend<T> for SegmentTree<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iterator = iter.into_iter();
        let (lower, upper) = iterator.size_hint();
        match upper.filter(|&u| u == lower) {
            Some(len) => self.extend_with_length(len, iterator),
            None => self.extend_from_slice(&iterator.collect::<Vec<_>>()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_monoid,
        monoid::OptionMonoid,
        op::{Coalesce, Gcd, Lcm, Max, Min, Prod, Sum, Xor},
    };

    use super::*;

    #[test]
    fn test_sum() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut sum_tree: SegmentTree<_> = data.into_iter().map(Sum).collect();
        assert_monoid!(&sum_tree[..]);

        assert_eq!(sum_tree.combine(3..5).0, 7);
        assert_eq!(sum_tree.combine(2..7).0, 20);
        assert_eq!(sum_tree.combine(..).0, 55);
        assert_eq!(sum_tree.combine(5..5).0, 0);
        sum_tree.update(5, 10.into());
        assert_eq!(sum_tree.combine(3..=4).0, 7);
        assert_eq!(sum_tree.combine(2..7).0, 25);
        assert_eq!(sum_tree.combine(1..).0, 60);
        sum_tree.update_with(7, |Sum(x)| Sum(x * 2)); // t[7] = 14
        assert_eq!(sum_tree.combine(..6).0, 20);
        assert_eq!(sum_tree.combine(6..=8).0, 28);
    }

    #[test]
    fn test_prod() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut prod_tree: SegmentTree<_> = data.into_iter().map(Prod).collect();
        assert_monoid!(&prod_tree[..]);

        assert_eq!(prod_tree.combine(3..5).0, 12);
        assert_eq!(prod_tree.combine(2..7).0, 720);
        assert_eq!(prod_tree.combine(0..11).0, 0);
        assert_eq!(prod_tree.combine(6..6).0, 1);
        prod_tree.update(5, 10.into());
        assert_eq!(prod_tree.combine(3..5).0, 12);
        assert_eq!(prod_tree.combine(2..7).0, 1440);
        assert_eq!(prod_tree.combine(0..).0, 0);
        prod_tree.update_with(7, |Prod(x)| Prod(x / 2)); // t[7] = 3
        assert_eq!(prod_tree.combine(5..=7).0, 180);
        assert_eq!(prod_tree.combine(8..).0, 720);
    }

    #[test]
    fn test_max() {
        let data = [2, -5, 122, -33, -12, 14, -55, 500, 3];
        let mut max_tree: SegmentTree<_> = data.into_iter().map(Max).collect();
        assert_monoid!(&max_tree[..]);

        assert_eq!(max_tree.combine(3..5).0, -12);
        assert_eq!(max_tree.combine(2..=6).0, 122);
        assert_eq!(max_tree.combine(..).0, 500);
        assert_eq!(max_tree.combine(0..0).0, i32::MIN);
        max_tree.update(5, 1000.into());
        assert_eq!(max_tree.combine(3..=4).0, -12);
        assert_eq!(max_tree.combine(2..7).0, 1000);
        assert_eq!(max_tree.combine(..10).0, 1000);
    }

    #[test]
    fn test_min() {
        let data = [2, -5, 122, 33, 12, 14, -55, 500, 3];
        let mut min_tree: SegmentTree<_> = data.into_iter().map(Min).collect();
        assert_monoid!(&min_tree[..]);

        assert_eq!(min_tree.combine(3..5).0, 12);
        assert_eq!(min_tree.combine(2..7).0, -55);
        assert_eq!(min_tree.combine(0..).0, -55);
        assert_eq!(min_tree.combine(5..5).0, i32::MAX);
        min_tree.update(5, (-1000).into());
        assert_eq!(min_tree.combine(3..5).0, 12);
        assert_eq!(min_tree.combine(2..7).0, -1000);
        assert_eq!(min_tree.combine(..10).0, -1000);
    }

    #[test]
    fn test_gcd() {
        let data = [10u32, 3, 4, 8, 6, 2];
        let mut gcd_tree: SegmentTree<_> = data.into_iter().map(Gcd).collect();
        assert_monoid!(&gcd_tree[..]);

        assert_eq!(gcd_tree.combine(2..4).0, 4);
        assert_eq!(gcd_tree.combine(2..6).0, 2);
        assert_eq!(gcd_tree.combine(0..6).0, 1);
        assert_eq!(gcd_tree.combine(3..3).0, 0);
        gcd_tree.update(5, 7.into());
        assert_eq!(gcd_tree.combine(2..4).0, 4);
        assert_eq!(gcd_tree.combine(2..6).0, 1);
        assert_eq!(gcd_tree.combine(0..6).0, 1);
    }

    #[test]
    fn test_lcm() {
        let data = vec![10u32, 3, 4, 8, 6, 2];
        let mut lcm_tree: SegmentTree<_> = data.into_iter().map(Lcm).collect();
        assert_monoid!(&lcm_tree[..]);

        assert_eq!(lcm_tree.combine(2..4).0, 8);
        assert_eq!(lcm_tree.combine(2..6).0, 24);
        assert_eq!(lcm_tree.combine(..).0, 120);
        assert_eq!(lcm_tree.combine(4..4).0, 1);
        lcm_tree.update(5, 7.into());
        assert_eq!(lcm_tree.combine(2..4).0, 8);
        assert_eq!(lcm_tree.combine(2..6).0, 168);
        assert_eq!(lcm_tree.combine(..).0, 840);
    }

    #[test]
    fn test_xor() {
        let data = [0b111, 0b101, 0b100, 0b000, 0b010];
        let mut xor_tree: SegmentTree<_> = data.into_iter().map(Xor).collect();
        assert_monoid!(&xor_tree[..]);
        assert_eq!(xor_tree.combine(2..4).0, 0b100);
        assert_eq!(xor_tree.combine(2..5).0, 0b110);
        assert_eq!(xor_tree.combine(0..5).0, 0b100);
        assert_eq!(xor_tree.combine(5..5).0, 0b000);
        xor_tree.update(4, 0b110.into());
        assert_eq!(xor_tree.combine(2..4).0, 0b100);
        assert_eq!(xor_tree.combine(2..5).0, 0b010);
        assert_eq!(xor_tree.combine(0..5).0, 0b000);
    }

    #[test]
    fn test_bisect() {
        let data = [-22, -5, 122, -33, -12, 14, -55, 500, 3];
        let mut max_tree: SegmentTree<_> = data.into_iter().map(Max).collect();
        assert_eq!(max_tree.bisect_left(2..5, |&Max(x)| x >= 10), Some(2));
        assert_eq!(max_tree.bisect_left(3..5, |&Max(x)| x >= 10), None);
        assert_eq!(max_tree.bisect_right(2..5, |&Max(x)| x >= 10), Some(2));
        assert_eq!(max_tree.bisect_right(3..5, |&Max(x)| x >= 10), None);
        max_tree.update(2, (-5).into());
        assert_eq!(max_tree.bisect_left(1..3, |&Max(x)| x >= -5), Some(1));
        assert_eq!(max_tree.bisect_left(1..5, |&Max(x)| x >= 500), None);
        assert_eq!(max_tree.bisect_left(5..10, |&Max(x)| x >= 500), Some(7));
        assert_eq!(max_tree.bisect_right(1..3, |&Max(x)| x >= -5), Some(2));
        assert_eq!(max_tree.bisect_right(1..5, |&Max(x)| x >= 500), None);
        assert_eq!(max_tree.bisect_right(5..10, |&Max(x)| x >= 500), Some(7));
        max_tree.update(3, (-5).into());
        assert_eq!(max_tree.bisect_left(..5, |&Max(x)| x >= -5), Some(1));
        assert_eq!(max_tree.bisect_right(..5, |&Max(x)| x >= -5), Some(3));
        max_tree.update(4, (-5).into());
        assert_eq!(max_tree.bisect_left(..5, |&Max(x)| x >= -5), Some(1));
        assert_eq!(max_tree.bisect_right(..5, |&Max(x)| x >= -5), Some(4));
        max_tree.update(0, (-5).into());
        assert_eq!(max_tree.bisect_left(..5, |&Max(x)| x >= -5), Some(0));
        assert_eq!(max_tree.bisect_right(..5, |&Max(x)| x >= -5), Some(4));
    }

    #[test]
    fn test_empty_tree() {
        let empty = SegmentTree::<OptionMonoid<Coalesce<u64>>>::from(vec![]);
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);
        assert_eq!(empty[..], vec![]);
        assert_eq!(
            empty.tree,
            vec![OptionMonoid::identity(), OptionMonoid::identity()]
        );
        assert_eq!(empty.combine(..), OptionMonoid::identity());
        assert_eq!(empty.combine(0..0), OptionMonoid::identity());
    }

    #[test]
    fn test_singleton_tree() {
        let mut single = SegmentTree::<_>::from(vec![OptionMonoid::from(Coalesce(Some(3)))]);
        assert!(!single.is_empty());
        assert_eq!(single.len(), 1);
        assert_eq!(single[..], vec![OptionMonoid::from(Coalesce(Some(3)))]);
        assert_eq!(
            single.tree,
            vec![
                OptionMonoid::identity(),
                OptionMonoid::from(Coalesce(Some(3)))
            ]
        );

        assert_eq!(single.combine(..), OptionMonoid::from(Coalesce(Some(3))));
        assert_eq!(single.combine(1..1), OptionMonoid::identity());
        assert_eq!(single.combine(1..), OptionMonoid::identity());
        single.update(0, OptionMonoid::from(Coalesce(Some(5))));
        assert_eq!(single.combine(..), OptionMonoid::from(Coalesce(Some(5))));
        assert_eq!(single.combine(1..), OptionMonoid::identity());
    }

    #[test]
    fn test_pair_tree() {
        let mut pair = SegmentTree::<_>::from(vec![
            OptionMonoid::from(Coalesce(Some(3))),
            OptionMonoid::from(Coalesce(Some(4))),
        ]);
        assert!(!pair.is_empty());
        assert_eq!(pair.len(), 2);
        assert_eq!(
            pair[..],
            vec![
                OptionMonoid::from(Coalesce(Some(3))),
                OptionMonoid::from(Coalesce(Some(4))),
            ]
        );
        assert_eq!(
            pair.tree,
            vec![
                OptionMonoid::identity(),
                OptionMonoid::from(Coalesce(Some(3))),
                OptionMonoid::from(Coalesce(Some(3))),
                OptionMonoid::from(Coalesce(Some(4))),
            ]
        );

        assert_eq!(pair.combine(..), OptionMonoid::from(Coalesce(Some(3))));
        assert_eq!(pair.combine(1..1), OptionMonoid::identity());
        assert_eq!(pair.combine(1..), OptionMonoid::from(Coalesce(Some(4))));
        pair.update(0, OptionMonoid::from(Coalesce(Some(5))));
        assert_eq!(pair.combine(..), OptionMonoid::from(Coalesce(Some(5))));
        assert_eq!(pair.combine(1..), OptionMonoid::from(Coalesce(Some(4))));
        assert_eq!(
            pair.tree,
            vec![
                OptionMonoid::identity(),
                OptionMonoid::from(Coalesce(Some(5))),
                OptionMonoid::from(Coalesce(Some(5))),
                OptionMonoid::from(Coalesce(Some(4))),
            ]
        );
    }

    #[test]
    fn test_large_tree() {
        let cum_sum = |s, t| (t - s + 1) * (s + t) / 2;
        let mut sum_tree: SegmentTree<_> = (0..2000000).map(Sum).collect();
        assert_eq!(sum_tree.combine(0..=10).0, cum_sum(0u128, 10u128));
        assert_eq!(sum_tree.combine(5..=15).0, cum_sum(5, 15));
        assert_eq!(sum_tree.combine(123..=1234567).0, cum_sum(123, 1234567));
        assert_eq!(sum_tree.combine(123456..=345678).0, cum_sum(123456, 345678));
        assert_eq!(sum_tree.combine(888888..=999999).0, cum_sum(888888, 999999));
        assert_eq!(sum_tree.combine(..).0, cum_sum(0, 1999999));

        for i in 0..2000000 {
            sum_tree.update_with(i, |Sum(x)| Sum(x + 1)); // expensive loop
        }
        let cum_sum_1 = |s, t| (t - s + 1) * (s + t + 2) / 2;
        assert_eq!(sum_tree.combine(0..=10).0, cum_sum_1(0u128, 10u128));
        assert_eq!(sum_tree.combine(5..=15).0, cum_sum_1(5, 15));
        assert_eq!(sum_tree.combine(123..=1234567).0, cum_sum_1(123, 1234567));
        assert_eq!(
            sum_tree.combine(123456..=345678).0,
            cum_sum_1(123456, 345678)
        );
        assert_eq!(
            sum_tree.combine(888888..=999999).0,
            cum_sum_1(888888, 999999)
        );
        assert_eq!(sum_tree.combine(..).0, cum_sum_1(0, 1999999));
    }

    #[test]
    fn test_push() {
        let cum_sum = |s, t| (t - s + 1) * (s + t) / 2;
        let mut sum_tree = SegmentTree::new();
        for i in 0..1023 {
            sum_tree.push(Sum(i));
            assert_eq!(sum_tree.combine(..).0, cum_sum(0, i));
        }
    }
    #[test]
    fn test_large_push() {
        let cum_sum = |s, t| (t - s + 1) * (s + t) / 2;
        let mut sum_tree = SegmentTree::new();
        for i in 0..2000000 {
            sum_tree.push(Sum(i)); // expensive loop
            assert_eq!(sum_tree.combine(..).0, cum_sum(0u128, i));
        }
    }

    #[test]
    fn test_extend() {
        let cum_sum = |s, t| (t - s + 1) * (s + t) / 2;
        let mut sum_tree = SegmentTree::new();
        sum_tree.extend((0..1).map(Sum));
        assert_eq!(sum_tree.combine(..).0, cum_sum(0, 0));
        sum_tree.extend((1..10).map(Sum));
        assert_eq!(sum_tree.combine(..).0, cum_sum(0, 9));
        sum_tree.extend((10..100).map(Sum));
        assert_eq!(sum_tree.combine(..).0, cum_sum(0, 99));
        sum_tree.extend((100..200).map(Sum));
        assert_eq!(sum_tree.combine(..).0, cum_sum(0, 199));
        sum_tree.extend((200..300).map(Sum));
        assert_eq!(sum_tree.combine(..).0, cum_sum(0, 299));
        sum_tree.extend((300..400).map(Sum));
        assert_eq!(sum_tree.combine(..).0, cum_sum(0, 399));
        sum_tree.extend((400..500).map(Sum));
        assert_eq!(sum_tree.combine(..).0, cum_sum(0, 499));
        sum_tree.extend((500..600).map(Sum));
        assert_eq!(sum_tree.combine(..).0, cum_sum(0, 599));
        sum_tree.extend((600..700).map(Sum));
        assert_eq!(sum_tree.combine(..).0, cum_sum(0, 699));
        sum_tree.extend((700..800).map(Sum));
        assert_eq!(sum_tree.combine(..).0, cum_sum(0, 799));
        sum_tree.extend((800..900).map(Sum));
        assert_eq!(sum_tree.combine(..).0, cum_sum(0, 899));
        sum_tree.extend((900..1000).map(Sum));
        assert_eq!(sum_tree.combine(..).0, cum_sum(0, 999));
        sum_tree.extend_from_slice(&[
            Sum(1000),
            Sum(1001),
            Sum(1002),
            Sum(1003),
            Sum(1004),
            Sum(1005),
            Sum(1006),
            Sum(1007),
            Sum(1008),
            Sum(1009),
        ]);
        assert_eq!(sum_tree.combine(..).0, cum_sum(0, 1009));
    }

    #[test]
    #[allow(clippy::reversed_empty_ranges)]
    fn test_descending_empty_range() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let sum_tree: SegmentTree<_> = data.into_iter().map(Sum).collect();
        assert_eq!(sum_tree.combine(10..0).0, 0);
        assert_eq!(sum_tree.combine(10..9).0, 0);
        assert_eq!(sum_tree.combine(9..8).0, 0);
    }
}
