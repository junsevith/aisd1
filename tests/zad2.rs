use rand::Rng;
use aisd1::cycle;

#[test]
fn test_cyclic_list() {
    println!("Cyclic list:");

    let mut list1 = cycle::CyclicList::new();
    let mut list2 = cycle::CyclicList::new();
    let mut rng = rand::thread_rng();
    for i in 0..10 {
        let number = rng.gen_range(10..99);
        list1.insert(number);
        println!("Added to list 1 {:?}", number);
    }
    for i in 0..10 {
        let number = rng.gen_range(10..99);
        list2.insert(number);
        println!("Added to list 2 {:?}", number);
    }
    list1.merge(list2);
    println!("Merged lists");
    println!("{:?}", list1.get_elems());
}
