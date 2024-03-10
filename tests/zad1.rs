use aisd1::queue;
use aisd1::stack;

#[test]
fn queue_test() {
    println!("Queue:");
    let mut queue = queue::Queue::new();
    for i in 0..50 {
        queue.push(i);
        println!("Added {:?}", i);
    }
    for _i in 0..50 {
        println!("{:?}", queue.pop());
    }
}

#[test]
fn stack_test() {
    println!("Stack:");

    let mut queue = stack::Stack::new();
    for i in 0..50 {
        queue.push(i);
        println!("Added {:?}", i);
    }
    for _i in 0..50 {
        println!("{:?}", queue.pop());
    }
}