use quickcheck_derive::Arbitrary;

#[derive(Arbitrary)]
enum E {
  #[quickcheck(bogus)]
  A,
  B,
}

fn main() {}
