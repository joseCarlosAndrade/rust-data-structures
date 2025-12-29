/*
when using Option, if we unwrap it directly, we consume it (move ownership). we cannot use that option again

let c = Option<Box<Node{}>>

if let Some(node) = c {
    // unwrap it 

}

we cannot use c again

/////// how to deal with it?
/// 
/// use references to the original value

let head = Option<Box<Node{}>>

let current : Option<&mut Box<Node>> = head.as_mut();


*/

struct Node {
    value : i32,
    next: Option<Box<Node>>
}

fn _a() {
    let mut head = Some(Box::new(Node{ value : 10, next: None}));

    let current : Option<&mut Box<Node>> = head.as_mut(); // borrowing as mutable
    // but!!!! instead of &mut Option<Box<Node>> we have Option<&mut Box<Node>>
    // its like creating another Option that has a mutable reference to whats inside the original box

    // now we can consume current and keep head intact

    if let Some(_node) = current {
        // do whatever with _node (even change it)
    }

    println!("{}", head.unwrap().value); // we can still use it
    
}

fn _using_take() {
    // in rust we cannot move out from a reference, nor leave a empty data for no time from pointers
    // like, when swapping pointers, there will be a minimal moment where one of the pointers will point to 
    // invalid data (dangling pointers). to prevent this, rust forbids this. we have to use take() on options
    // which moves the value to the assignemnt var and leaves a None in the original place (this way, no invalid
    // memory is allowed)

    // ex :  head(1, next(2, None))
    let mut head = Node 
        { 
            value : 1, 
            next : Some(
                Box::new(
                    Node
                    {
                        value : 2, 
                        next : None
                    })
                )
        };

    let node = &mut head;

    
    // let stolen = node.next; // error: cannot move out of node.next bc its behind a shared reference
    // let stolen = head.next; // allowed, bc head owns the value. however, since Node does not implement
                                                // the Copy trait, it will move the value from head to stolen

    let _stolen = node.next.take(); // laves node.next as None, while moving the value to stolen 
}