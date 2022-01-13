use ahecha_macro::Validate;

#[test]
fn validator_test() {
  #[derive(Validate)]
  struct Test {
    #[validate(length(min = 10, max = 20, message = "A test"))]
    field1: String,
  }
  assert!(false);
}
