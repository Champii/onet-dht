use std::sync::{ Arc, Mutex };

#[derive(Clone, Debug, Default)]
pub struct Mutexed<T> {
  pub mutex: Arc<Mutex<T>>,
}

impl<T: Clone> Mutexed<T> {
  pub fn new(t: T) -> Mutexed<T> {
    Mutexed {
      mutex: Arc::new(Mutex::new(t)),
    }
  }

  #[allow(dead_code)]
  pub fn set(&mut self, t: T) {
    let mut guard = self.mutex.lock().unwrap();

    *guard = t;
  }

  pub fn get(&self) -> T {
    let guard = self.mutex.lock().unwrap();

    (*guard).clone()
  }

  pub fn map(&self, f: Box<Fn(&mut T)>) {
    let mut guard = self.mutex.lock().unwrap();

    f(&mut *guard);
  }

  #[allow(dead_code)]
  pub fn map_result<R>(&self, f: Box<Fn(&mut T) -> R>) -> R{
    let mut guard = self.mutex.lock().unwrap();

    f(&mut *guard)
  }
}
