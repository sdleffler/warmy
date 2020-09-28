//! Shareable resources.

#[cfg(not(feature = "arc"))]
use std::{
  cell::{Ref, RefCell, RefMut},
  rc::Rc,
};
#[cfg(feature = "arc")]
use {
  atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut},
  std::sync::Arc,
};

/// Shareable resource type.
///
/// Resources are wrapped in this type. You cannot do much with an object of this type, despite
/// borrowing immutable or mutably its content.
#[derive(Debug)]
pub struct Res<T>(ResInner<T>);

#[cfg(feature = "arc")]
type ResInner<T> = Arc<AtomicRefCell<T>>;

#[cfg(not(feature = "arc"))]
type ResInner<T> = Rc<RefCell<T>>;

impl<T> Clone for Res<T> {
  fn clone(&self) -> Self {
    Res(self.0.clone())
  }
}

#[cfg(feature = "arc")]
impl<T> Res<T> {
  /// Wrap a value in a shareable resource.
  pub fn new(t: T) -> Self {
    Res(Arc::new(AtomicRefCell::new(t)))
  }

  /// Borrow a resource for as long as the return value lives.
  pub fn borrow(&self) -> AtomicRef<T> {
    self.0.borrow()
  }

  /// Mutably borrow a resource for as long as the return value lives.
  pub fn borrow_mut(&self) -> AtomicRefMut<T> {
    self.0.borrow_mut()
  }
}

#[cfg(not(feature = "arc"))]
impl<T> Res<T> {
  /// Wrap a value in a shareable resource.
  pub fn new(t: T) -> Self {
    Res(Rc::new(RefCell::new(t)))
  }

  /// Borrow a resource for as long as the return value lives.
  pub fn borrow(&self) -> Ref<T> {
    self.0.borrow()
  }

  /// Mutably borrow a resource for as long as the return value lives.
  pub fn borrow_mut(&self) -> RefMut<T> {
    self.0.borrow_mut()
  }
}
