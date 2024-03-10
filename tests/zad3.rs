use rand::prelude::IndexedRandom;
use rand::Rng;
use rand::rngs::ThreadRng;

use aisd1::double;

#[test]
fn merge_test() {
    println!("Double linked circular list:");
    let mut list1 = double::CyclicList::new();
    let mut list2 = double::CyclicList::new();
    let mut rng = rand::thread_rng();
    for _i in 0..10 {
        let number = rng.gen_range(10..99);
        list1.insert(number);
        println!("Added to list 1 {:?}", number);
    }
    for _i in 0..10 {
        let number = rng.gen_range(10..99);
        list2.insert(number);
        println!("Added to list 2 {:?}", number);
    }
    list1.merge(list2);
    println!("Merged lists");
    println!("{:?}", list1.get_elems());
}

#[test]
fn lookup_test() {
    let mut list = double::CyclicList::new();
    let mut nums = Vec::new();
    let mut rng = rand::thread_rng();
    for _i in 0..10_000 {
        let number = rng.gen_range(0..100_000);
        list.insert(number);
        nums.push(number);
        // println!("Added to list {:?}", number);
    }

    let mut avg = 0;
    for _i in 0..1000 {
        let rand = nums.choose(&mut rng).unwrap();
        avg += random_find(&list, *rand, &mut rng);
    }
    println!("Average 1: {:?}", avg / 1000);

    let mut avg = 0;
    for _i in 0..1000 {
        let rand = rng.gen_range(0..100_000);
        avg += random_find(&list, rand, &mut rng);
    }
    println!("Average 2: {:?}", avg / 1000);
}

fn random_find(list: &double::CyclicList<i32>, num: i32, rng: &mut ThreadRng) -> usize {
    if rng.gen_bool(0.5) {
        // println!("Forward");
        list.find_forward(num).unwrap()
    } else {
        // println!("Backward");
        list.find_backward(num).unwrap()
    }
}