use quickcheck_richderive::quickcheck;

fn my_gen(_g: &mut quickcheck::Gen) -> i32 {
  0
}

#[quickcheck(z = "my_gen")]
fn t(a: i32, b: String) -> bool {
  let _ = (a, b);
  true
}

fn main() {}
