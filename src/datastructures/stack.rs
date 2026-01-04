use std::fmt::{Display, write};

const MAX_ELEMENTS : usize = 100;

pub struct Stack {
    count : usize,
    elements : Vec<i32>,
}

#[derive(Debug)]
pub enum StackError {
    CapacityFull,
    StackEmpty,
}

impl Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{}", )
        match self {
            StackError::CapacityFull => write!(f, "stack at full capacity"),
            StackError::StackEmpty => write!(f, "stack is empty")
        }
    }
}

impl Stack {
    pub fn new() -> Self {
        Self {
            count : 0,
            elements : vec![],
        }
    }

    pub fn push(&mut self, value : i32) -> Result<(), StackError> {
        if self.count >= MAX_ELEMENTS {
            return Err(StackError::CapacityFull);
        }

        // todo
        self.elements.push(value);

        self.count += 1;

        Ok(())
    }

    pub fn pop(&mut self) -> Result<i32, StackError> {
        if self.count == 0 {
            return Err(StackError::StackEmpty);
        }

        let i= self.elements.pop().unwrap();

        self.count -= 1;

        Ok(i)
    }

    pub fn peek(&self) -> Result<i32, StackError> {
        if self.count == 0 {
            return Err(StackError::StackEmpty);
        }

        let i = self.elements[self.count-1];

        Ok(i)
    }

    pub fn print(&self) {
        for i in &self.elements {
            println!("{}", i);
        }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}