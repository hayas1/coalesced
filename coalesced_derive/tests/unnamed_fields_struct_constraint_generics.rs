// TODO required phantom data and conflict type parameter

// use std::fmt::Display;

// use coalesced::{Coalesce, Extension};

// #[derive(Coalesce)]
// struct Config<S, T: Clone>(S, Option<T>)
// where
//     S: Display + Extension<E>;

// #[test]
// fn test_derive_coalesce_unnamed_fields_struct_constraint_generics() {
//     let config = Config("c1", Some(1));
//     let config2 = Config("c2", None);

//     let c = config.prior(config2);
//     assert_eq!(c.0, "c2");
//     assert_eq!(c.1, Some(1));
// }

// #[test]
// fn test_derive_extension_unnamed_fields_struct_constraint_generics() {
//     let config = Config("c1", None).with_extension("first");
//     let config2 = Config("c2", Some(2)).with_extension("second");

//     let c = config.posterior(config2);
//     assert_eq!(c.0.extension, "first");
//     assert_eq!(*c.0, "c1");
//     assert_eq!(c.1.extension, "second");
//     assert_eq!(*c.1, Some(2));

//     assert!(matches!(c.into(), Config("c1", Some(2))));
// }
