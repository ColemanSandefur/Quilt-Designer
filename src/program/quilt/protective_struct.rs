//
// ProtectiveStructure
//
// Used to provide flexible access to an internal variable,
// Easily lets you run code before and after access to an internal variable (See Block for example)
//

pub trait ProtectiveStructure<T> {
    fn modify(&mut self, modification: impl FnOnce(&mut T));
}