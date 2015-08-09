mod check;
mod assert_bool;
mod assert_array;
mod assert_int;

fn main() {
  assert_bool::run();
  assert_array::run();
  assert_int::run();
  println!("All Koans passing!");
}
