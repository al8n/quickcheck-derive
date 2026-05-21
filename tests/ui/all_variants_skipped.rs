use quickcheck_richderive::Arbitrary;

#[derive(Arbitrary)]
enum E {
  #[quickcheck(skip)]
  A,
  #[quickcheck(skip)]
  B,
}

fn main() {}
