// Having these features on means the generated code will use them
// which in turns requires us to have the crates as dev-dependencies.
// None of these tests depend any of the functionality, so I'd rather
// just not run the tests in the case any of them are activated.
// Tests are run with default-features enabled, which is in CI.
#[cfg(not(any(
    feature = "serde",
    feature = "glam",
    feature = "mpi",
    feature = "hdf5",
    feature = "rand",
    feature = "num-traits-libm",
)))]
mod tests {
    #[test]
    #[cfg(feature = "f64")]
    fn compile_fail_float() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_fail/float_*.rs");
    }

    #[test]
    #[cfg(feature = "f64")]
    fn compile_fail_resolver() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_fail/resolver_*.rs");
    }

    #[test]
    #[cfg(feature = "f64")]
    #[cfg(not(feature = "rational-dimensions"))]
    fn compile_fail_type_mismatch() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_fail/type_mismatch_*.rs");
    }

    #[test]
    fn compile_fail_dimension_def_numeric_factor() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_fail/dimension_definition_with_numeric_factor.rs");
    }

    #[test]
    fn compile_fail_dimension_annotation() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_fail/dimension_annotation*.rs");
    }
}
