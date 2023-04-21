pub trait Wrapper<T>: From<T> + Into<T> {
  fn raw(&self) -> &T;
}
