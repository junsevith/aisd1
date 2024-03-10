pub struct Stack<T> {
    top: Option<Box<StackElement<T>>>,

}

struct StackElement<T> {
    element: T,
    next: Option<Box<StackElement<T>>>,
}

impl <T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            top: None,
        }
    }
    pub fn push(&mut self, element: T) {
        self.top = Some(Box::new(StackElement::new(element, self.top.take())));

    }
    pub fn pop(&mut self) -> Option<T> {
        match self.top.take() {
            Some(element) => {
                self.top = element.next;
                Some(element.element)
            }
            None => None,
        }
    }
}

impl <T> StackElement<T> {
    fn new(element: T, next: Option<Box<StackElement<T>>>) -> StackElement<T> {
        StackElement {
            element,
            next,
        }
    }
}