pub fn run() {
  check(index());
  check(empty());
}

fn check(ans: bool) {
  assert!(ans, "Failure");
  print!("✓");
}

// The elements of an array can be accessed by their indices
// arr[4]
fn index() -> bool {
  let arr: [i32; 5] = [1, 2, 3, 4, 5];
  arr__ == 1
}

// A new fixed size array can be created by declaring
// the type of its elements along with its length
// [i32; 0] = []
fn empty() -> bool {
  let arr: __;
  arr.len() == 0
}
