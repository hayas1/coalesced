use std::path::Path;

#[test]
fn test_ui() {
    let t = trybuild::TestCases::new();
    #[cfg(not(feature = "histogram"))]
    {
        prepare_rs_file("tests/ui/compile_fail", "tests/ui/semigroup").unwrap();
        t.compile_fail("tests/ui/semigroup/**/*.rs");
    }
    #[cfg(feature = "histogram")]
    {
        prepare_rs_file("tests/ui/compile_fail", "tests/ui/histogram").unwrap();
        t.compile_fail("tests/ui/histogram/**/*.rs");
    }
}

fn prepare_rs_file<P: AsRef<Path>>(source: P, target: P) -> Result<(), std::io::Error> {
    let files = std::fs::read_dir(source.as_ref())?;
    files
        .filter_map(Result::ok)
        .filter(|f| f.path().extension().map(|e| e == "rs").unwrap_or(false))
        .try_for_each(|f| {
            let t = target.as_ref().join(f.file_name());
            std::fs::copy(f.path(), &t).map(|_| ())
        })
}
