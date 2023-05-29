use ll_stack::{GenericStack, Iterators};
use stack_trait::Stack;

// A trait which implements the print marker: `{:?}`.
use core::fmt::Debug;
use std::fmt;
use std::fmt::Display;

/// A simple struct to demonstrate how to use of the `ll_stack` implementation
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Du64 {
    value: u64,
}

impl Drop for Du64 {
    fn drop(&mut self) {
        println!("Dropped: {}", self.value);
    }
}

impl Display for Du64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

fn main() {
    // Create a new stack for elements of type struct Du64
    let mut stack = GenericStack::new();

    // Check empty stack behaves right
    assert_eq!(stack.pop(), None);

    // Populate stack
    stack.push(Du64 { value: 1 });
    stack.push(Du64 { value: 2 });
    stack.push(Du64 { value: 3 });
    stack.push(Du64 { value: 4 });
    stack.push(Du64 { value: 5 });

    // We can print the stack using trait Debug
    println!("stack  {stack:?}");
    // We can clone the stack since ll_stack implements trait Clone
    let stack2 = stack.clone();
    // We print the cloned stack using the Debug trait.
    // The Debug trait was automatically implemented by rustc
    println!("stack2 {stack2:?}");
    // We print the stack using the Display trait that we explicitly implemented
    // - this looks differently
    println!("stack  {stack}");

    // ll_stack implements the PartialEq trait and hence,
    // we can compare two stacks. The cloned stack2 must be
    // identical with the original stack.
    assert_eq!(stack, stack2);

    // pop and print the top element
    println!("Top element: {:?}", stack.pop());

    // peek (i.e., borrow the element without removing from the stack)
    // and print the top element
    println!("Top element: {:?}", stack.peek());

    // peek_mut let's us modify the top element
    // we need to ensure that this element exists
    if let Some(node) = stack.peek_mut() {
        node.value += 1;
    }

    // print the new top element - must be changed
    println!("Top element: {:?}", stack.peek());

    // print the whole stack using the Debug trait
    println!("Stack {stack:?}");

    // print all the elements of the stack by iterating
    // over all elements and printing the elements using
    // the Debug trait of the elements
    for v in stack.iter() {
        println!("Value: {v:?}");
    }

    println!("Incrementing all values of the stack...");
    for v in stack.iter_mut() {
        v.value += 1;
        println!("Incremented Value: {v:?}");
    }

    println!("Into Iterator - removes from stack");
    println!("stack={stack}");
    for mut v in stack.into_iter() {
        v.value += 1;
        println!("Incremented Value: {v:?}");
    }

    let mut stack = GenericStack::new();
    if let Some(value) = stack.peek_mut() {
        *value += 1;
    }

    // Populate stack
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    stack.push(5);
    stack.iter().min();
    println!("stack  {stack}: minimum={:?}", stack.iter().min());
    println!("stack  {stack}: maximum={:?}", stack.iter().max());
    println!("stack  {stack}: sum={:?}", stack.iter().sum::<u128>());

    let mut entries = 0;
    for _e in stack.iter() {
        entries += 1;
    }
    println!("Entries in stack: {entries}");

    // a map without a consumer of the iterator causes a warning
    // we switch of a warning
    #[allow(unused_must_use)]
    {
        stack.iter_mut().map(|value| *value += 1);
    }
    stack.iter_mut().map(|value| *value += 1).last();

    // Count the number of entries by iterating over all elements of the stack
    let mut entries = 0;
    stack.iter().map(|_| entries += 1).last();
    println!("Entries in stack: {entries}");

    // Count the number of entries using the count function
    println!("Entries in stack: {}", stack.iter().count());
    let stack: GenericStack<u128> = GenericStack::new();
    // Sum up all the elements in the stack using the sum function
    println!("stack  {stack}: sum={:?}", stack.iter().sum::<u128>());
}
