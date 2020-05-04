pub trait Map {
    type Key;
    type Value;

    fn new() -> Self;
    fn get(&self, key: &Self::Key) -> Option<*const Self::Value>;
    fn insert(&mut self, key: Box<Self::Key>, value: *const Self::Value);
    fn remove_entry(&mut self, key: &Self::Key) -> Option<(Box<Self::Key>, *const Self::Value)>;
}
