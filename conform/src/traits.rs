use std::marker::Sized;

/// The trait implemented by `conform_derive`
pub trait Conform {
  fn conform(&mut self) -> &mut Self
  where
    Self: Sized;
}
