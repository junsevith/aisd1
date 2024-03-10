use rand::prelude::IndexedRandom;
use rand::Rng;
use aisd1::cycle;

#[test]
fn merge_test() {
    println!("Cyclic list:");

    let mut list1 = cycle::CyclicList::new();
    let mut list2 = cycle::CyclicList::new();
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
    println!();
    println!("Looking for elements in list");
    let mut list = cycle::CyclicList::new();
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
        avg += list.find(*rand).unwrap();
    }
    println!("Average 1: {:?}", avg / 1000);

    let mut avg = 0;
    for _i in 0..1000 {
        let rand = rng.gen_range(0..100_000);
        avg += list.find(rand).unwrap();
    }
    println!("Average 2: {:?}", avg / 1000);
}