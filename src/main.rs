mod assert_bool;
mod assert_array;
mod check;

fn main() {
  assert_bool::run();
  assert_array::run();
  println!("All Koans passing!");
}
