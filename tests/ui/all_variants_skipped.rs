use quickcheck_derive::Arbitrary;

#[derive(Arbitrary)]
enum E {
  #[quickcheck(skip)]
  A,
  #[quickcheck(skip)]
  B,
}

fn main() {}
