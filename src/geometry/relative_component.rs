pub(crate) trait RelativeComponent<I, O> {

  fn get_absolute(&self, t: &I) -> O;
}