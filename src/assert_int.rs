use check;

pub fn run() {
  check::check(unsigned_ints());
  check::check(signed_ints());
}

// As the name implies, unsigned integers (u8, u16, u32, u64) cannot be negative
fn unsigned_ints() -> bool {
  u8::min_value() __ 0
}

// Signed integers(i8, i16, i32, i64), on the other hand, can be negative
fn signed_ints() -> bool {
  i8::min_value() __ 0
}
