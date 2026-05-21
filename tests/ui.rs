//! Compile-fail UI tests for the derive's diagnostics.
//!
//! Each case pins a `compile_error!` the macro emits itself (via `syn::Error`),
//! so the expected `.stderr` is the macro's own message text — stable across
//! rustc versions. Regenerate expectations with:
//!
//! ```sh
//! TRYBUILD=overwrite cargo test --test ui
//! ```

#[test]
fn ui() {
  let t = trybuild::TestCases::new();
  t.compile_fail("tests/ui/*.rs");
}
