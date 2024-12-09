pub trait Edge<T> {
    fn add(&mut self, id: &str, value: T);
    fn delete(&mut self, id: &str);
}
