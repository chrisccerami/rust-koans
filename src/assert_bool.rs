pub fn truthiness() -> bool {
  assert!(truth(), "Truth should be true");
  assert!(truth(), "Falsehood should be false");
  true
}

fn truth() -> bool {
  true == true
}

fn falsehood() -> bool {
  false == true
}
