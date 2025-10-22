use crate::ListMut;

pub trait ListResizable<T>: ListMut<T> {
    fn resize(&mut self, new_len: usize, value: T)
    where
        T: Clone;
    fn resize_with<F>(&mut self, new_len: usize, f: F)
    where
        F: FnMut() -> T;
    fn reserve(&mut self, additional: usize);
    fn shrink_to_fit(&mut self);
}
