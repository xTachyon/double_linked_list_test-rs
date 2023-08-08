mod solutions;
use std::time::Instant;

use solutions::nonnull_impl::DoubleLinkedList;

fn main() {
    let mut d = DoubleLinkedList::new();
    let start_time = Instant::now();
    for index in 1..100_000 {
        d.add(index % 2);
    }
    println!("Create time: {:?}",Instant::now()-start_time);
    let start_time = Instant::now();
    let sum = d.sum_all();
    println!("iteration time: {:?}",Instant::now()-start_time);
    println!("Sum = {sum}");

}

