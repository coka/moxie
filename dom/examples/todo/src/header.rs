use {
    super::*,
    moxie_dom::{elements::*, events::*, *},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Header {
    todos: Key<Vec<Todo>>,
}

impl Header {
    pub fn new(todos: Key<Vec<Todo>>) -> Self {
        Self { todos }
    }
}

impl Component for Header {
    fn contents(self) {
        // show!(header().class_name("header").contains(sibs![
        //     h1().contains(text!("todos")),
        //     // TodoTextInput::new().on_save()
        // ]))
    }
}
