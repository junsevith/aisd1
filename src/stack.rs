pub(crate) struct Queue<T> {
    first: Option<Box<QueueElement<T>>>,

}

struct QueueElement<T> {
    element: T,
    next: Option<Box<QueueElement<T>>>,
}

impl <T> Queue<T> {
    pub(crate) fn new() -> Queue<T> {
        Queue {
            first: None,
        }
    }
    pub(crate) fn enqueue(&mut self, element: T) {
        self.first = Some(Box::new(QueueElement::new(element, self.first.take())));

    }
    pub(crate) fn dequeue(&mut self) -> Option<T> {
        match self.first.take() {
            Some(element) => {
                self.first = element.next;
                Some(element.element)
            }
            None => None,
        }
    }
}

impl <T> QueueElement<T> {
    fn new(element: T, next: Option<Box<QueueElement<T>>>) -> QueueElement<T> {
        QueueElement {
            element,
            next,
        }
    }
}