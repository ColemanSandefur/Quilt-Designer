pub trait ProtectiveStructure<T> {
    fn modify(&mut self, modification: impl FnOnce(&mut T));
}