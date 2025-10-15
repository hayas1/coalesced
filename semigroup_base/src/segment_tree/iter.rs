use crate::segment_tree::SegmentTree;

impl<T> IntoIterator for SegmentTree<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(mut self) -> Self::IntoIter {
        let (leaf_offset, len) = (self.leaf_offset(), self.len());
        let mut leaf = self.tree.split_off(leaf_offset);
        let _out_of_range = leaf.split_off(len);
        IntoIter {
            inner: leaf.into_iter(),
        }
    }
}
impl<'a, T> IntoIterator for &'a SegmentTree<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner: self[..].iter(),
        }
    }
}
impl<T> SegmentTree<T> {
    /// **O(1)**, get iterator of the segment tree leaf.
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            inner: self[..].iter(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct IntoIter<T> {
    inner: <Vec<T> as IntoIterator>::IntoIter,
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}
impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

pub struct Iter<'a, T> {
    inner: <&'a [T] as IntoIterator>::IntoIter,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}
impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::{monoid::OptionMonoid, op::overwrite::Overwrite};

    use super::*;

    #[test]
    fn test_into_iter() {
        let segment_tree: SegmentTree<_> = ["one", "two", "three", "four", "five"]
            .into_iter()
            .map(Overwrite)
            .map(OptionMonoid::from)
            .collect();
        let v: Vec<_> = segment_tree
            .into_iter()
            .map(|x| match x {
                OptionMonoid(Some(Overwrite(s))) => s,
                _ => unreachable!(),
            })
            .collect();
        assert_eq!(v, ["one", "two", "three", "four", "five"]);
    }

    #[test]
    fn test_iter() {
        let segment_tree: SegmentTree<_> = ["one", "two", "three", "four", "five"]
            .into_iter()
            .map(Overwrite)
            .map(OptionMonoid::from)
            .collect();
        let v: Vec<_> = segment_tree
            .iter()
            .map(|x| match x {
                OptionMonoid(Some(Overwrite(s))) => s,
                _ => unreachable!(),
            })
            .collect();
        assert_eq!(v, [&"one", &"two", &"three", &"four", &"five"]);
    }

    #[test]
    fn test_for() {
        let segment_tree: SegmentTree<_> = ["one", "two", "three", "four", "five"]
            .into_iter()
            .map(Overwrite)
            .map(OptionMonoid::from)
            .collect();
        let mut v = Vec::new();
        for OptionMonoid(x) in &segment_tree {
            match x {
                Some(Overwrite(s)) => v.push(s),
                _ => unreachable!(),
            }
        }
        assert_eq!(v, [&"one", &"two", &"three", &"four", &"five"]);
    }

    #[test]
    fn test_double_ended_iter() {
        let segment_tree: SegmentTree<_> = ["one", "two", "three", "four", "five"]
            .into_iter()
            .map(Overwrite)
            .map(OptionMonoid::from)
            .collect();
        let v: Vec<_> = segment_tree
            .iter()
            .rev()
            .map(|x| match x {
                OptionMonoid(Some(Overwrite(s))) => s,
                _ => unreachable!(),
            })
            .collect();
        assert_eq!(v, [&"five", &"four", &"three", &"two", &"one"]);

        let v: Vec<_> = segment_tree
            .into_iter()
            .rev()
            .map(|x| match x {
                OptionMonoid(Some(Overwrite(s))) => s,
                _ => unreachable!(),
            })
            .collect();
        assert_eq!(v, ["five", "four", "three", "two", "one"]);
    }

    #[test]
    fn test_exact_size_iter() {
        let segment_tree: SegmentTree<_> = ["one", "two", "three", "four", "five"]
            .into_iter()
            .map(Overwrite)
            .map(OptionMonoid::from)
            .collect();
        assert_eq!(segment_tree.iter().len(), 5);
        assert_eq!(segment_tree.into_iter().len(), 5);
    }
}
