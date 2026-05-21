use quickcheck_richderive::Arbitrary;

#[derive(Arbitrary)]
enum E {
  #[quickcheck(bogus)]
  A,
  B,
}

fn main() {}
