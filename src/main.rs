use rand::Rng;

mod cycle;

fn main() {
    let mut list = cycle::CyclicList::new();
    let mut nums = Vec::new();
    let mut rng = rand::thread_rng();
    println!("Lookup test");
    for _i in 0..10_000 {
        let number = rng.gen_range(0..100_000);
        list.insert(number);
        nums.push(number);
        // println!("Added to list {:?}", number);
    }

    // let mut avg = 0;
    // for _i in 0..1000 {
    //     let rand = nums.choose(&mut rng).unwrap();
    //     avg += list.find(*rand).unwrap();
    // }
    // println!("Average 1: {:?}", avg / 1000);
    //
    // let mut avg = 0;
    // for _i in 0..1000 {
    //     let rand = rng.gen_range(0..100_000);
    //     avg += list.find(rand).unwrap();
    // }
    // println!("Average 2: {:?}", avg / 1000);
}