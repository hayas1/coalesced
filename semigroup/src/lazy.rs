use crate::Semigroup;

pub trait Combine<T: Semigroup> {
    fn combine(self) -> T;
    fn combine_rev(self) -> T;
    fn combine_cloned(&self) -> T
    where
        Self: Clone,
    {
        self.clone().combine()
    }
    fn combine_rev_cloned(&self) -> T
    where
        Self: Clone,
    {
        self.clone().combine_rev()
    }
}

macro_rules! lazy_tuple_impl {
    ($($idx:tt $t:tt),+) => {
        impl<T: Semigroup, $( $t: Into<T> ),+> Combine<T> for ($($t,)+) {
            fn combine(self) -> T {
                let v = vec![$(self.$idx.into()),+];
                v.into_iter().reduce(|a, b| a.semigroup(b)).unwrap_or_else(|| unreachable!())
            }
            fn combine_rev(self) -> T {
                let v = vec![$(self.$idx.into()),+];
                v.into_iter().rev().reduce(|a, b| a.semigroup(b)).unwrap_or_else(|| unreachable!())
            }
        }
    };
}

lazy_tuple_impl!(0 A);
lazy_tuple_impl!(0 A, 1 B);
lazy_tuple_impl!(0 A, 1 B, 2 C);
lazy_tuple_impl!(0 A, 1 B, 2 C, 3 D);
