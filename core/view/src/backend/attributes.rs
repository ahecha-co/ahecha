pub trait Attributes: Default + Clone {
  type Builder;
  fn builder() -> Self::Builder;
}

impl Attributes for () {
  type Builder = ();
  fn builder() -> Self::Builder {
    ()
  }
}
