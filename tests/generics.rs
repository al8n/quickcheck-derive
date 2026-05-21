//! Generic structs: inferred bounds and explicit `bound = "..."`.

use quickcheck::{Arbitrary, Gen};
use quickcheck_derive::Arbitrary as DeriveArbitrary;

fn gen() -> Gen {
  Gen::new(16)
}

// Inferred bound: `T: ::quickcheck::Arbitrary`.
#[derive(Clone, Debug, PartialEq, DeriveArbitrary)]
struct Wrapper<T> {
  inner: T,
  tag: u8,
}

#[test]
fn generic_inferred_bound() {
  let mut g = gen();
  let value: Wrapper<Vec<u8>> = Wrapper::arbitrary(&mut g);
  let probe = Wrapper {
    inner: value.inner.clone(),
    tag: value.tag,
  };
  let _shrinks: Vec<Wrapper<Vec<u8>>> = probe.shrink().collect();
}

// Explicit (odd) bound combined with a `default` field so the type need not
// itself be `Arbitrary` — proves `bound` replaces the inferred bound.
#[derive(Clone, Debug, PartialEq, DeriveArbitrary)]
#[quickcheck(bound = "T: Clone + Default + 'static")]
struct Holder<T> {
  #[quickcheck(default)]
  inner: T,
  n: u16,
}

#[derive(Clone, Debug, PartialEq, Default)]
struct NotArbitrary(String);

#[test]
fn explicit_bound_with_default_field() {
  let mut g = gen();
  // `NotArbitrary` is NOT `Arbitrary`; compiles only because the bound is
  // `T: Clone + Default` and the field uses `default`.
  let value: Holder<NotArbitrary> = Holder::arbitrary(&mut g);
  assert_eq!(value.inner, NotArbitrary::default());
  let _shrinks: Vec<Holder<NotArbitrary>> = value.shrink().collect();
}

// Two generic params, both inferred.
#[derive(Clone, Debug, PartialEq, DeriveArbitrary)]
struct Pair<A, B> {
  a: A,
  b: B,
}

#[test]
fn two_param_generic() {
  let mut g = gen();
  let value: Pair<u8, bool> = Pair::arbitrary(&mut g);
  let _ = value.shrink().count();
}
