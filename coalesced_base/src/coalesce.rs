pub trait Coalesce {
    fn prior(self, other: Self) -> Self;
    fn posterior(self, other: Self) -> Self;
}

impl<T> Coalesce for T {
    default fn prior(self, _other: Self) -> Self {
        self
    }
    default fn posterior(self, other: Self) -> Self {
        other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coalesce_option() {
        assert_eq!(None::<Option<()>>, None.prior(None));
        assert_eq!(Some(1), None.prior(Some(1)));
        assert_eq!(Some(1), Some(1).prior(None));
        assert_eq!(Some(2), Some(1).prior(Some(2)));
        assert_eq!(
            Some(4),
            None.prior(Some(2)).prior(None).prior(Some(4)).prior(None)
        );

        assert_eq!(None::<Option<()>>, None.posterior(None));
        assert_eq!(Some(1), None.posterior(Some(1)));
        assert_eq!(Some(1), Some(1).posterior(None));
        assert_eq!(Some(1), Some(1).posterior(Some(2)));
        assert_eq!(
            Some(2),
            None.posterior(Some(2))
                .posterior(None)
                .posterior(Some(4))
                .posterior(None)
        );
    }

    #[test]
    fn test_coalesce_result() {
        assert_eq!(Err::<(), _>("one"), Err("one").prior(Err("two")));
        assert_eq!(Ok(2), Err("one").prior(Ok(2)));
        assert_eq!(Ok(1), Ok(1).prior(Err("two")));
        assert_eq!(Ok::<_, ()>(2), Ok(1).prior(Ok(2)));
        assert_eq!(
            Ok(4),
            Err("one")
                .prior(Ok(2))
                .prior(Err("three"))
                .prior(Ok(4))
                .prior(Err("five"))
        );

        assert_eq!(Err::<(), _>("two"), Err("one").posterior(Err("two")));
        assert_eq!(Ok(2), Err("one").posterior(Ok(2)));
        assert_eq!(Ok(1), Ok(1).posterior(Err("two")));
        assert_eq!(Ok::<_, ()>(1), Ok(1).posterior(Ok(2)));
        assert_eq!(
            Ok(2),
            Err("one")
                .posterior(Ok(2))
                .posterior(Err("three"))
                .posterior(Ok(4))
                .posterior(Err("five"))
        );
    }

    #[test]
    fn test_coalesce_integer() {
        assert_eq!(0, 0.prior(0));
        assert_eq!(1, 0.prior(1));
        assert_eq!(0, 1.prior(0));
        assert_eq!(2, 1.prior(2));
        assert_eq!(0, 0.prior(2).prior(0).prior(4).prior(0));

        assert_eq!(0, 0.posterior(0));
        assert_eq!(0, 0.posterior(1));
        assert_eq!(1, 1.posterior(0));
        assert_eq!(1, 1.posterior(2));
        assert_eq!(0, 0.posterior(2).posterior(0).posterior(4).posterior(0));
    }

    #[test]
    fn test_coalesce_str() {
        assert_eq!("", "".prior(""));
        assert_eq!("foo", "".prior("foo"));
        assert_eq!("", "foo".prior(""));
        assert_eq!("bar", "foo".prior("bar"));
        assert_eq!("", "".prior("bar").prior("").prior("baz").prior(""));

        assert_eq!("", "".posterior(""));
        assert_eq!("", "".posterior("foo"));
        assert_eq!("foo", "foo".posterior(""));
        assert_eq!("foo", "foo".posterior("bar"));
        assert_eq!(
            "",
            "".posterior("bar")
                .posterior("")
                .posterior("baz")
                .posterior("")
        );
    }

    #[test]
    fn test_coalesce_ref() {
        assert_eq!(&0, (&0).prior(&0));
        assert_eq!(&false, (&false).posterior(&true));
    }
}
