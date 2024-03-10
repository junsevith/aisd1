pub struct Queue<T> {
    front: Option<Box<QueueElement<T>>>,
    back: Option<*mut QueueElement<T>>,
}

struct QueueElement<T> {
    element: T,
    next: Option<Box<QueueElement<T>>>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            front: None,
            back: None,
        }
    }

    pub fn push(&mut self, element: T) {
        let mut new_element = Box::new(QueueElement::new(element));
        let raw_pointer = new_element.as_mut() as *mut QueueElement<T>;
        match self.back {
            None => {
                match self.front {
                    None => {
                        self.front = Some(new_element);
                    }
                    Some(ref mut front) => {
                        front.set_next(Some(new_element));
                    }
                }
                self.back = Some(raw_pointer);
            }
            Some(back) => {
                unsafe {
                    (*back).set_next(Some(new_element));
                }
                self.back = Some(raw_pointer);
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.front.take() {
            None => None,
            Some(front) => {
                let element = front.element;
                self.front = front.next;
                if self.front.is_none() {
                    self.back = None;
                }
                Some(element)
            }
        }
    }
}

impl<T> QueueElement<T> {
    fn new(element: T) -> QueueElement<T> {
        QueueElement {
            element,
            next: None,
        }
    }

    fn set_next(&mut self, next: Option<Box<QueueElement<T>>>) {
        self.next = next;
    }
}