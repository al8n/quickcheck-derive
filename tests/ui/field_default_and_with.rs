use quickcheck_derive::Arbitrary;

#[derive(Arbitrary)]
struct S {
  #[quickcheck(default, with = "whatever")]
  x: u8,
}

fn main() {}
