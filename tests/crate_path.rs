//! `#[quickcheck(crate = "...")]` redirects the emitted `Arbitrary`/`Gen` path.

mod my_qc {
  pub use quickcheck::*;
}

use quickcheck_derive::Arbitrary as DeriveArbitrary;

#[derive(Clone, Debug, PartialEq, DeriveArbitrary)]
#[quickcheck(crate = "crate::my_qc")]
struct Via {
  a: u8,
  b: Vec<u8>,
}

#[test]
fn custom_crate_path_compiles_and_runs() {
  // Reference the trait through the same alias to exercise the impl.
  use crate::my_qc::Arbitrary;
  let mut g = my_qc::Gen::new(16);
  let value = Via::arbitrary(&mut g);
  let _shrinks: Vec<Via> = value.shrink().collect();

  let probe = Via {
    a: 3,
    b: vec![1, 2],
  };
  for s in probe.shrink() {
    let changed = (s.a != probe.a) as u32 + (s.b != probe.b) as u32;
    assert_eq!(changed, 1);
  }
}

// Enum through the alias too.
#[derive(Clone, Debug, PartialEq, DeriveArbitrary)]
#[quickcheck(crate = "crate::my_qc")]
enum ViaEnum {
  A,
  B(u8),
}

#[test]
fn custom_crate_path_enum() {
  use crate::my_qc::Arbitrary;
  let mut g = my_qc::Gen::new(16);
  let _ = ViaEnum::arbitrary(&mut g);
  assert_eq!(ViaEnum::A.shrink().count(), 0);
}
