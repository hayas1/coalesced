use std::{ops::Index, slice::SliceIndex};

use crate::segment_tree::SegmentTree;

impl<T> SegmentTree<T> {
    /// **O(1)**, get element(s) by index like [`Vec`].
    pub fn get<I: SegmentTreeIndex<T>>(&self, index: I) -> Option<&I::Output> {
        // index.get(self) // warn: a method with this name may be added to the standard library in the future
        SegmentTreeIndex::get(index, self)
    }
}
impl<T, I: SegmentTreeIndex<T>> Index<I> for SegmentTree<T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        SegmentTreeIndex::index(index, self)
    }
}

pub trait SegmentTreeIndex<T> {
    type Output: ?Sized;

    fn get(self, segment_tree: &SegmentTree<T>) -> Option<&Self::Output>;
    // TODO unsafe fn get_unchecked(self, segment_tree: *const SegmentTree<T>) -> *const Self::Output;
    fn index(self, segment_tree: &SegmentTree<T>) -> &Self::Output;
}

impl<T, I: SliceIndex<[T]>> SegmentTreeIndex<T> for I {
    type Output = I::Output;

    fn get(self, segment_tree: &SegmentTree<T>) -> Option<&Self::Output> {
        let (leaf_offset, len) = (segment_tree.leaf_offset(), segment_tree.len());
        if len == 0 {
            segment_tree.tree[0..0].get(self) // for 1-indexed default tree
        } else {
            segment_tree.tree[leaf_offset..leaf_offset + len].get(self)
        }
    }
    fn index(self, segment_tree: &SegmentTree<T>) -> &Self::Output {
        let (leaf_offset, len) = (segment_tree.leaf_offset(), segment_tree.len());
        if len == 0 {
            &segment_tree.tree[0..0][self] // for 1-indexed default tree
        } else {
            &segment_tree.tree[leaf_offset..leaf_offset + len][self]
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{monoid::OptionMonoid, op::Overwrite};

    use super::*;

    #[test]
    fn test_index() {
        let mut segment_tree: SegmentTree<_> = ["one", "two", "three", "four", "five"]
            .into_iter()
            .map(Overwrite)
            .map(OptionMonoid::from)
            .collect();
        assert_eq!(segment_tree[1], OptionMonoid::from(Overwrite("two")));
        assert_eq!(segment_tree[2..2], []);
        assert_eq!(
            segment_tree[1..3],
            [
                OptionMonoid::from(Overwrite("two")),
                OptionMonoid::from(Overwrite("three")),
            ]
        );
        assert_eq!(
            segment_tree[1..=3],
            [
                OptionMonoid::from(Overwrite("two")),
                OptionMonoid::from(Overwrite("three")),
                OptionMonoid::from(Overwrite("four")),
            ]
        );
        assert_eq!(
            segment_tree[..2],
            [
                OptionMonoid::from(Overwrite("one")),
                OptionMonoid::from(Overwrite("two")),
            ]
        );
        assert_eq!(
            segment_tree[..=2],
            [
                OptionMonoid::from(Overwrite("one")),
                OptionMonoid::from(Overwrite("two")),
                OptionMonoid::from(Overwrite("three")),
            ]
        );
        assert_eq!(segment_tree[4..], [OptionMonoid::from(Overwrite("five"))]);
        assert_eq!(segment_tree[5..], []);
        segment_tree.update(2, OptionMonoid::from(Overwrite("3")));
        assert_eq!(
            segment_tree[..],
            [
                OptionMonoid::from(Overwrite("one")),
                OptionMonoid::from(Overwrite("two")),
                OptionMonoid::from(Overwrite("3")),
                OptionMonoid::from(Overwrite("four")),
                OptionMonoid::from(Overwrite("five")),
            ]
        );
    }

    #[rstest]
    #[case::too_large_usize(100)]
    #[case::too_long_range(3..100)]
    #[case::out_of_range(6..)]
    #[should_panic]
    fn test_index_panic<I: SegmentTreeIndex<OptionMonoid<Overwrite<&'static str>>>>(
        #[case] index: I,
    ) {
        let segment_tree: SegmentTree<_> = ["one", "two", "three", "four", "five"]
            .into_iter()
            .map(Overwrite)
            .map(OptionMonoid::from)
            .collect();
        let _ = segment_tree[index];
    }

    #[test]
    fn test_get() {
        let mut segment_tree: SegmentTree<_> = ["one", "two", "three", "four", "five"]
            .into_iter()
            .map(Overwrite)
            .map(OptionMonoid::from)
            .collect();
        assert_eq!(
            segment_tree.get(1),
            Some(&OptionMonoid::from(Overwrite("two")))
        );
        assert_eq!(segment_tree.get(2..2), Some(&[][..]));
        assert_eq!(segment_tree.get(100), None);
        assert_eq!(
            segment_tree.get(1..3),
            Some(
                &[
                    OptionMonoid::from(Overwrite("two")),
                    OptionMonoid::from(Overwrite("three")),
                ][..]
            )
        );
        assert_eq!(
            segment_tree.get(1..=3),
            Some(
                &[
                    OptionMonoid::from(Overwrite("two")),
                    OptionMonoid::from(Overwrite("three")),
                    OptionMonoid::from(Overwrite("four")),
                ][..]
            )
        );
        assert_eq!(segment_tree.get(3..100), None);
        assert_eq!(
            segment_tree.get(..2),
            Some(
                &[
                    OptionMonoid::from(Overwrite("one")),
                    OptionMonoid::from(Overwrite("two")),
                ][..]
            )
        );
        assert_eq!(
            segment_tree.get(..=2),
            Some(
                &[
                    OptionMonoid::from(Overwrite("one")),
                    OptionMonoid::from(Overwrite("two")),
                    OptionMonoid::from(Overwrite("three")),
                ][..]
            )
        );
        assert_eq!(
            segment_tree.get(4..),
            Some(&[OptionMonoid::from(Overwrite("five"))][..])
        );
        assert_eq!(segment_tree.get(5..), Some(&[][..]));
        assert_eq!(segment_tree.get(6..), None);
        segment_tree.update(2, OptionMonoid::from(Overwrite("3")));
        assert_eq!(
            segment_tree.get(..),
            Some(
                &[
                    OptionMonoid::from(Overwrite("one")),
                    OptionMonoid::from(Overwrite("two")),
                    OptionMonoid::from(Overwrite("3")),
                    OptionMonoid::from(Overwrite("four")),
                    OptionMonoid::from(Overwrite("five")),
                ][..]
            )
        );
    }
}
