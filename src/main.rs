mod datastructures;

use datastructures::{LinkedList};

fn main() {
    println!("Hello, world!");

    do_linked_lists();
}

fn do_linked_lists() {
    let mut linked_list = LinkedList::new();

    linked_list.push(10);
    linked_list.push(11);
    linked_list.push(12);
    linked_list.push_back(100);
    linked_list.push_back(99);
    linked_list.push_back(98);


    linked_list.print();
}