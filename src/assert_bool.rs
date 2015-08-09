pub fn run() {
  assert!(truth(), "Truth should be true");
  assert!(falsehood(), "Falsehood should be false");
}

fn truth() -> bool {
  true == true
}

fn falsehood() -> bool {
  !(false == true)
}
