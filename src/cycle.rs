use std::cell::RefCell;
use std::rc::Rc;

pub struct CyclicList<T> {
    head: Option<Rc<CycleElement<T>>>,
    size: usize,
}

struct CycleElement<T> {
    element: T,
    next: RefCell<Option<Rc<CycleElement<T>>>>,
}

impl<T> CyclicList<T> {
    pub fn new() -> CyclicList<T> {
        CyclicList {
            head: None,
            size: 0,
        }
    }

    pub fn insert(&mut self, element: T) {
        let element = CycleElement::new(element);
        match self.head {
            Some(ref head) => {
                match &*head.next.borrow() {
                    Some(ref next) => {
                        element.connect(next);
                    }
                    None => {
                        element.connect(head)
                    }
                }
                head.connect(&element);
            }
            None => {}
        }
        self.head = Some(Rc::clone(&element));
    }

    pub fn merge(&mut self, mut second: CyclicList<T>) {
        println!("Merging lists");
        match self.head {

            // Lista ma przynajmniej jeden element (head)
            Some(ref first_end) => {
                match second.head {

                    // Druga lista ma przynajmniej jeden element (head)
                    Some(ref second_end) => {
                        let first_beginning;
                        let second_beginning;

                        match &*first_end.next.borrow() {

                            // Pierwsza lista ma przynajmniej dwa elementy
                            Some(ref next) => {
                                first_beginning = next.clone();
                            }
                            // Pierwsza lista ma tylko jeden element
                            None => {
                                first_beginning = first_end.clone();
                            }
                        }

                        match &*second_end.next.borrow() {

                            // Druga lista ma przynajmniej dwa elementy
                            Some(ref next) => {
                                second_beginning = next.clone();
                            }

                            // Druga lista ma tylko jeden element
                            None => {
                                second_beginning = second_end.clone();
                            }
                        }

                        first_end.connect(&second_beginning);
                        second_end.connect(&first_beginning);

                        self.head = second.head.clone();
                    }
                    // Druga lista jest pusta
                    None => {}
                }
            }
            // Lista jest pusta
            None => {
                self.head = second.head.clone();
            }
        }
        // we need to reset the second list head because otherwise 
        // dropping 2nd list will drop elements from the merged list
        second.head = None;
    }
}

impl<T: Clone> CyclicList<T> {
    pub fn get_elems(&self) -> Vec<T> {
        let mut result = Vec::new();
        let mut count: usize = 0;
        match self.head.as_ref() {
            None => {}
            Some(head) => {
                let mut current = head.clone();
                loop {
                    result.push(current.element.clone());
                    count += 1;
                    if count > 20 {
                        break;
                    }
                    match current.next() {
                        None => {
                            break;
                        }
                        Some(next) => {
                            current = next;
                        }
                    }
                    if Rc::ptr_eq(&current, &head) {
                        break;
                    }
                }
            }
        }
        return result;
    }
}

impl<T: Eq> CyclicList<T> {
    pub fn find(&self, template: T) -> Option<usize> {
        let mut count: usize = 0;
        match self.head.as_ref() {
            None => { None }
            Some(head) => {
                let mut current = head.clone();
                loop {
                    if current.element == template {
                        return Some(count);
                    }
                    match current.next() {
                        None => {
                            // return None;
                            return Some(count);
                        }
                        Some(next) => {
                            current = next;
                        }
                    }
                    count += 1;
                    if Rc::ptr_eq(&current, &head) {
                        // return None;
                        return Some(count);
                    }
                }
            }
        }
    }
}

impl<T> Drop for CyclicList<T> {
    fn drop(&mut self) {
        match self.head {
            None => {}
            Some(ref head) => {
                // when dropping Cyliclist we need to break the cycle 
                // (delete one of the references)
                // otherwise cyclic references will cause memory leaks
                head.next.take();
            }
        }
    }
}

impl<T> CycleElement<T> {
    fn new(element: T) -> Rc<CycleElement<T>> {
        Rc::new(CycleElement {
            element,
            next: RefCell::new(None),
        })
    }

    fn next(&self) -> Option<Rc<Self>> {
        match *self.next.borrow() {
            Some(ref x) => Some(x.clone()),
            None => None
        }
    }

    fn connect(&self, next: &Rc<Self>) {
        self.next.replace(Some(Rc::clone(next)));
    }
}