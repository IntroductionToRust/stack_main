# `stack_main`

This repo defines a simple program that shows how to use the linked-list-based, monomorphic stack implementation `ll_stack`.

This code is used in the context of the following class:

- *Introduction to programming with Rust* (see https://tu-dresden.de/ing/informatik/sya/se/studium/lehrveranstaltungen/summer-semester/programming/index).

- the lecture slides are/will be available on GitHub <https://github.com/IntroductionToRust/lecture-slides>.

## Building

To check, build, test, and document, just execute:

```bash
./build.sh
```

This command will fail, in case not all required tools are installed. Please install the missing tools with the help of `cargo`.

## Documentation

After building the project, you can view the documentation in your browser by opening the following file `target/doc/stack_main/index.html` or by executing:

```bash
cargo doc --open
```

## GitHub Pages

The documentation of this project is available at <https://introductiontorust.github.io/stack_main>.

## Output of program

When you run this program with `cargo run`, you can expect the following output:

```text
stack  GenericStack { head: Some(Node { element: Du64 { value: 5 }, next: Some(Node { element: Du64 { value: 4 }, next: Some(Node { element: Du64 { value: 3 }, next: Some(Node { element: Du64 { value: 2 }, next: Some(Node { element: Du64 { value: 1 }, next: None }) }) }) }) }) }
stack2 GenericStack { head: Some(Node { element: Du64 { value: 5 }, next: Some(Node { element: Du64 { value: 4 }, next: Some(Node { element: Du64 { value: 3 }, next: Some(Node { element: Du64 { value: 2 }, next: Some(Node { element: Du64 { value: 1 }, next: None }) }) }) }) }) }
stack  head->5->4->3->2->1.
Top element: Some(Du64 { value: 5 })
Dropped: 5
Top element: Some(Du64 { value: 4 })
Top element: Some(Du64 { value: 5 })
Stack GenericStack { head: Some(Node { element: Du64 { value: 5 }, next: Some(Node { element: Du64 { value: 3 }, next: Some(Node { element: Du64 { value: 2 }, next: Some(Node { element: Du64 { value: 1 }, next: None }) }) }) }) }
Value: Du64 { value: 5 }
Value: Du64 { value: 3 }
Value: Du64 { value: 2 }
Value: Du64 { value: 1 }
Incrementing all values of the stack...
Incremented Value: Du64 { value: 6 }
Incremented Value: Du64 { value: 4 }
Incremented Value: Du64 { value: 3 }
Incremented Value: Du64 { value: 2 }
Into Iterator - removes from stack
stack=head->6->4->3->2.
Incremented Value: Du64 { value: 7 }
Dropped: 7
Incremented Value: Du64 { value: 5 }
Dropped: 5
Incremented Value: Du64 { value: 4 }
Dropped: 4
Incremented Value: Du64 { value: 3 }
Dropped: 3
stack  head->5->4->3->2->1.: minimum=Some(1)
stack  head->5->4->3->2->1.: maximum=Some(5)
stack  head->5->4->3->2->1.: sum=15
Entries in stack: 5
Entries in stack: 5
Entries in stack: 5
stack  head.: sum=0
Dropped: 5
Dropped: 4
Dropped: 3
Dropped: 2
Dropped: 1
```
