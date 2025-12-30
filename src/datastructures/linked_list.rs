// todo list
/*
int linked list
generic linked list

operations:
- add elements
- remove last elements
*/

const MAX_ELEMENTS : usize = 100;

pub struct LinkedList {
    count : usize, // 8 bytes bc im in a 64 bits system
    head : Option<Box<Node>>, // Box is a smart pointer that owns an alocated value of T on the heap (Node here)
    // this way, this struct is fixed size. Pointer: 8 bytes
    // Box is a pointer stored in the stack that owns a value alocated in the heap
}

struct Node {
    next : Option<Box<Node>>,
    value: i32,
}

impl Node {
    fn create(value : i32) -> Node {
        Self { next : None, value : value}
    }
}


impl LinkedList {
    pub fn new() -> Self {
        LinkedList {
            count: 0,
            head: None,
        }
    }

    pub fn print(&mut self) {
        println!("=== begin list ===");
        if self.count == 0 {
            println!("=== end list ===");

            return
        }

        let mut current = self.head.as_mut();

        loop {
            println!("{}", current.as_ref().unwrap().value);

            if current.as_ref().unwrap().next.is_none() {
                break;
            }

            current = current.unwrap().next.as_mut();
        }

        println!("=== end list ===");
    }

    pub fn push(&mut self, value : i32) -> bool {

        let new = Box::new(
                Node { 
                    value: value, 
                    next : self.head.take() // take: in rust we cannot leave a hole in a struct. we must immediatley put something back on. take does that, it moves the value to the variable we're assigin and put a None in the original place. Its like swapping pointer in C, but at once
                }
            );

        self.head = Some(new);
        
        self.count += 1;
        true
    }

    
    pub fn push_back(&mut self, value : i32) -> bool {
        if self.count >= MAX_ELEMENTS {return false}

        if self.count == 0 {
            self.head = Some(Box::new(Node::create(value)));    
            self.count += 1;
            
            return true;
        }

        // let mut current = self.head.unwrap(); // this consumes the option and moves the value. it breaks the list

        // let current = self.head.as_ref(); // this gets a referece to whats inside the Option
                                             // Option<&Box <Node>>

        let mut current   = self.head.as_mut(); // so we get a mutable reference of
                                                                        // whats inside the Option without taking ownership
                                                                        // we get Option<&mut Box<Node>>
                                         

        // traverses the list until current.next is None
        while current
                .as_ref() // downs current as referece, otherwise the .unwrap() would consume the previous refrence and we wouldnt be able to use it inside the while loop
                .unwrap() 
                .next
                .is_some() {
                    current = current.unwrap().next.as_mut(); // we need ot keep a mutable reference again
                }
        
        current.unwrap().next = Some(Box::new(Node::create(value)));
        
        self.count += 1;
        
        /*
        todo: test this
        let mut current = self.head.as_mut();

        // While 'current' contains a 'Some' that has a 'next' that is also 'Some'
        while let Some(node) = current {
            if node.next.is_none() {
                // We found the end!
                node.next = Some(Box::new(Node { value: val, next: None }));
                break; 
            }
            // Move to the next link
            current = node.next.as_mut();
        }
     */



        true
    } 


    pub fn insert(&mut self, value: i32, pos : usize) -> bool {
        // inserts value at pos index
        true
    }

    pub fn pop(&mut self) -> Option<i32> {
        // pops the head
        if self.count <= 0 {
            return None;
        }

        let old_head = self.head.take().unwrap(); // self.head-> is now None
        self.head = old_head.next;

        self.count -= 1;

        // self.head.as_ref().unwrap().value
        Some(old_head.value) // theres another way of doing it like the function below
    }

    pub fn pop_back(&mut self) -> Option<i32> {
        // traverses the list and then pops it
        if self.count <= 0 {
            return None;
        }

        if self.count == 1 {
            self.count -= 1;

            return self.head.take().map(|node| return node.value); // edge case: pops head
        }

        let mut current = self.head.as_mut();

        while current.as_ref().unwrap().next.is_some() && 
                current.as_ref().unwrap().next.as_ref().unwrap().next.is_some() {

            current = current.unwrap().next.as_mut();

        }

        // this
        // let old_last = current.unwrap().next.take();
        // old_last.unwrap().value

        // or this
        return current.unwrap().next.take().map(|node| return node.value);
    }

    pub fn peek(&self) -> i32 {
        // peeks at the head but doesnt remove it
        self.head.as_ref().unwrap().value
    }
}