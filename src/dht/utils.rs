use std::sync::{ Arc, Mutex, RwLock };

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

  pub fn map_result<R>(&self, f: Box<Fn(&mut T) -> R>) -> R{
    let mut guard = self.mutex.lock().unwrap();

    f(&mut *guard)
  }
}

#[derive(Clone, Debug, Default)]
pub struct RwMutexed<T> {
  pub mutex: Arc<RwLock<T>>,
}

impl<T: Clone> RwMutexed<T> {
  pub fn new(t: T) -> RwMutexed<T> {
    RwMutexed {
      mutex: Arc::new(RwLock::new(t)),
    }
  }

  pub fn set(&mut self, t: T) {
    let mut guard = self.mutex.write().unwrap();

    *guard = t;
  }

  pub fn set_fn(&mut self, t: T, f: Box<Fn(&mut T)>) {
    let mut guard = self.mutex.write().unwrap();

    f(&mut *guard);
  }

  pub fn get(&self) -> T {
    let guard = self.mutex.read().unwrap();

    (*guard).clone()
  }

  pub fn get_fn<R>(&self, f: Box<Fn(&T) -> R>) -> R {
    let guard = self.mutex.read().unwrap();

    f(&*guard)
  }
}
