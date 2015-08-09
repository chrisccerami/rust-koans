pub fn run() {
  assert!(empty(), "");
}

fn empty() -> bool {
  let arr: [i32; 0] = [];
  let arr_two: [i32; 0] = [];
  arr == arr_two
}
