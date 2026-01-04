mod datastructures;

use std::arch::x86_64;

use datastructures::{LinkedList, Stack};

fn main() {
    println!("Hello, world!");

    // _do_linked_lists();
    _do_stacks();
}

fn _do_linked_lists() {
    let mut linked_list = LinkedList::new();

    linked_list.push(10);
    linked_list.push(11);
    linked_list.push(12);
    linked_list.push_back(100);
    linked_list.push_back(99);
    linked_list.push_back(98);


    linked_list.print();

    println!("\npopping");
    let v = linked_list.pop();
    linked_list.print();
    println!("\nvalued popped: {}", v.unwrap());

    let p = linked_list.peek();
    println!("\npeeking: {}", p);
    linked_list.print();


    println!("\npopping back:");
    let b = linked_list.pop_back();
    linked_list.print();
    println!("popped: {}", b.unwrap());

    println!("\n adding again: 90");
    linked_list.push(90);
    linked_list.print();
}

fn _do_stacks() {

    let mut stack = Stack::new();

    let _ = stack.push(10);
    let _ = stack.push(100);

    if let Err(e) = stack.push(99) {
        println!("could not push: {}", e);
    } else {
        println!("sucessfully pushed");
    }

    stack.print();

    let x = stack.pop().unwrap();
    let _ = stack.pop();
    let _ = stack.pop();
    println!("popped: {}", x);

    if let Err(e) = stack.peek() {
        println!("could not peek: {}", e);
    } 
}