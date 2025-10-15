#[derive(Debug, Clone, PartialEq)]
pub struct Name(pub &'static str);
macro_rules! var_name {
    ($var:ident) => {{
        let _ = &$var; // to follow the renaming
        crate::name::Name(stringify!($var))
    }};
}
pub(crate) use var_name;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Default)]
    struct Struct {
        foo: (),
        bar: (),
        baz: (),
    }

    #[test]
    fn test_var_name() {
        let foo = ();
        assert_eq!(var_name!(foo), Name("foo"));
        let bar = ();
        assert_eq!(var_name!(bar), Name("bar"));
        let baz = ();
        assert_eq!(var_name!(baz), Name("baz"));
    }

    #[test]
    fn test_field_name() {
        let Struct { foo, bar, baz } = Struct::default();
        assert_eq!(var_name!(foo), Name("foo"));
        assert_eq!(var_name!(bar), Name("bar"));
        assert_eq!(var_name!(baz), Name("baz"));
    }

    // #[test]
    // fn test_type_name() {
    //     assert_eq!(name!(Attr), Name("Attr"));
    // }
}
