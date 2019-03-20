use crate::{Entry, Instance};

impl<E: Entry> Instance<E> {}

impl<E: Entry> Drop for Instance<E> {
    fn drop(&mut self) {
        unsafe {
            (self.raw().destroy_instance)(self.as_raw());
        }
    }
}
