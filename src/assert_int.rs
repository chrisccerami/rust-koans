use check;

pub fn run() {
  check::check(unsigned_ints());
  check::check(signed_ints());
  check::check(sub_unsigned_int());
  check::check(sub_signed_int());
  check::check(add_numbers());
}

// As the name implies, unsigned integers (u8, u16, u32, u64) cannot be negative
fn unsigned_ints() -> bool {
  u8::min_value() == 0
}

// Unsigned integers can be reduced only as far as their minimum value
fn sub_unsigned_int() -> bool {
  let mut num:u8 = 10;
  num -= 10;
  num == u8::min_value()
}

// Signed integers(i8, i16, i32, i64), on the other hand, can be negative
fn signed_ints() -> bool {
  i8::min_value() < 0
}

// Signed integers can be reduced below zero, as far as their minimum value
fn sub_signed_int() -> bool {
  let mut num:i8 = 0;
  let neg:i8 = -128;
  num += neg;
  num == i8::min_value()
}

// Addition of positive integers works much the same for signed and unsigned numbers
fn add_numbers() -> bool {
  let mut sig:i8 = 0;
  let mut unsig:u8 = 0;
  sig += 127;
  unsig += 255;
  (sig == i8::max_value()) && (unsig == u8::max_value())
}
