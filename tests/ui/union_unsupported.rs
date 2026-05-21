use quickcheck_derive::Arbitrary;

#[derive(Arbitrary)]
union U {
  a: u32,
}

fn main() {}
