

pub struct UndoRedo<T> {
    undo: Vec<T>,
    redo: Vec<T>,
}

const MAX_SIZE: usize = 15;

impl<T> UndoRedo<T> {

    pub fn new() -> Self {
        Self {
            undo: Vec::with_capacity(MAX_SIZE),
            redo: Vec::with_capacity(MAX_SIZE),
        }
    }

    pub fn changed(&mut self, item: T) {
        UndoRedo::add_to_stack(&mut self.undo, item);
        self.redo.clear();
    }

    pub fn undo(&mut self, item: T) -> Option<T> {
        let popped = self.undo.pop();

        if let Some(_) = &popped {
            UndoRedo::add_to_stack(&mut self.redo, item);
        }

        popped
    }

    pub fn peek_undo(&self) -> Option<&T> {
        self.undo.last()
    }

    pub fn redo(&mut self, item: T) -> Option<T> {
        let popped = self.redo.pop();

        if let Some(_) = &popped {
            UndoRedo::add_to_stack(&mut self.undo, item);
        }

        popped
    }

    pub fn peek_redo(&self) -> Option<&T> {
        self.redo.last()
    }

    fn add_to_stack(stack: &mut Vec<T>, item: T) {
        if stack.len() == MAX_SIZE {
            stack.pop();
        }

        stack.push(item);
    }
}