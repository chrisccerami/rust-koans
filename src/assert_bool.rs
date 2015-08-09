use check;

pub fn run() {
  check::check(truth());
  check::check(falsehood());
  check::check(string_equality());
  check::check(int_equality());
}

// Booleans can have two values, true or false.
// Two equal values will return true when compared with the == operator
fn truth() -> bool {
  true == __
}

// Likewise, two unequal values will return false when compared with ==
fn falsehood() -> bool {
  !(false == __)
}

// Strings can also be compared and will return a boolean
fn string_equality() -> bool {
  "Stuff" == __
}

// Integers can be compared as long as they are of the same type
fn int_equality() -> bool {
  let num:i8 = 5;
  num == __
}
