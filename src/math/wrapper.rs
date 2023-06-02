pub trait Wrapper<T>: From<T> + Into<T> {
  fn from_inner(inner: T) -> Self { From::from(inner) }

  fn into_inner(self) -> T { self.into() }

  fn inner(&self) -> &T;
}
