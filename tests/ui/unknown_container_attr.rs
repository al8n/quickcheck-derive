use quickcheck_derive::Arbitrary;

#[derive(Arbitrary)]
#[quickcheck(bogus = "x")]
struct S {
  x: u8,
}

fn main() {}
