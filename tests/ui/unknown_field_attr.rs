use quickcheck_derive::Arbitrary;

#[derive(Arbitrary)]
struct S {
  #[quickcheck(bogus)]
  x: u8,
}

fn main() {}
