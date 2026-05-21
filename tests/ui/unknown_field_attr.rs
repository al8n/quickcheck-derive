use quickcheck_richderive::Arbitrary;

#[derive(Arbitrary)]
struct S {
  #[quickcheck(bogus)]
  x: u8,
}

fn main() {}
