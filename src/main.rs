mod assert_bool;

#[test]
fn it_works() {
}

fn main() {
  if assert_bool::truthiness() == true {
    println!("The truth is true")
  } else {
    println!("The truth is somehow false")
  }
}
