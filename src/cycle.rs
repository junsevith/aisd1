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

impl<T> CyclicList<T>{
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
                element.connect(&head.next());
                head.connect(&element);
            }
            None => {
                element.connect(&element)
            }
        }
        self.head = Some(element);
    }

    pub fn merge(&mut self, mut second: CyclicList<T>) {
        match self.head {

            // Lista ma przynajmniej jeden element (head)
            Some(ref first_head) => {
                match second.head {

                    // Druga lista ma przynajmniej jeden element (head)
                    Some(ref second_head) => {
                        let first_beginning = first_head.next();
                        let second_beginning = second_head.next();

                        first_head.connect(&second_beginning);
                        second_head.connect(&first_beginning);

                        self.head = second.head.take();
                    }
                    // Druga lista jest pusta
                    None => {}
                }
            }
            // Lista jest pusta
            None => {
                self.head = second.head.take();
            }
        }
        // we need to reset the second list head because otherwise 
        // dropping 2nd list will drop elements from the merged list
        // second.head = None;
    }
}

impl<T: Clone> CyclicList<T> {
    pub fn get_elems(&self) -> Vec<T> {
        let mut result = Vec::new();
        match self.head.as_ref() {
            None => {}
            Some(head) => {
                let mut current = head.next();
                while !Rc::ptr_eq(&current,&head){
                    result.push(current.element.clone());
                    current = current.next();
                }
                result.push(head.element.clone())
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
                    current = current.next();
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
        // println!("Dropping list");
        match self.head {
            None => {}
            Some(ref head) => {
                // when dropping Cyliclist we need to break the cycle
                // (delete one of the references)
                // otherwise cyclic references will cause memory leaks
                // self.head.take(); // old way of breaking the cycle

                //this code has pretty weird function
                //if we try to break the cycle for list size >10_000
                //the recursive calls for drop inside Rc causes stack overflow
                //so we need to partition the list and call for drop of only 1000 elements at one time
                if self.size > 1000 {
                    let mut i = 0;
                    let mut partitions = Vec::new();
                    let mut current = self.head.take().unwrap();
                    while i < self.size {
                        current = current.skip(i);
                        partitions.push(current.clone());
                        i += 1000;
                    }
                }

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

    ///returns copy of the pointer to the next element
    fn next(&self) -> Rc<Self> {
        //all list elements outside (part of)  insert function must have next element
        self.next.borrow().clone().unwrap()
    }

    fn connect(&self, next: &Rc<Self>) {
        //sets next element as specified in function
        self.next.replace(Some(Rc::clone(next)));
    }

    fn skip(&self,mut index: usize) -> Rc<CycleElement<T>>{
        let mut current = self.next();
        while index > 0 {
            current = current.next()
        }
        return current;
    }
}

// impl<T> Drop for CycleElement<T> {
//     fn drop(&mut self) {
//         println!("Dropping element");
//     }
//
// }