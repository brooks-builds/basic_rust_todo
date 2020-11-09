# basic_rust_todo

## User stories

- [x] As a doer, I want to be able to see my todo items
  - [x] initialize a new binary rust application
  - [x] Open todos.data file for reading
  - [x] put all items into a vector
  - [x] print the items to the screen
- [x] As a doer, I want to be able to add an todo item
  - [x] Figure out what the command is
  - [x] Open file for writing
  - [x] Append todo item to file
  - [x] new todo has correct format in file
- [x] As a doer, I want to mark a todo item as completed
- [x] As a doer, I want to mark a completed todo item as not completed
- [x] As a doer, I want to delete a todo item
- [x] Refactor
  - [x] DRY up command
  - [x] DRY up library for writing to file
  - [x] Change done to be a toggle
  - [x] qa tests use unwrap_err
- [x] QA
  - [x] test for not providing anything after a command that requires an id
  - [x] test for not providing number as an id
  - [x] test for providing to high of an id
  - [x] test for providing a negative id
  - [x] test for not providing a todo after command add

## How to use

List all todo items

```
basic_rust_todo
```

## Topics

<!-- - x Introduction to the course
- x Why Learn Rust -->
<!-- - First steps -->
  <!-- -x Installing Rust
    -x Windows
    -x Mac
    -x Linux -->
  <!-- -x  Hello World
  - Setting up VS Code
    - RLS
    - Rust Analyzer
    - Clippy
    - Rust FMT -->

- Code Basics
  - Comments (https://doc.rust-lang.org/reference/comments.html?)
    - // normal comments
    - /// doc comments
    - //! inner line doc comment (top of the file)
  - Printing to the screen
    - println!
    - dbg!
  - when to use semi-colons
    - implicit return
  - Variables
    - immutable vs mutable
    - reference
  - functions
    - ownership
    - borrowing
  - Data structures
    - Vectors
    - Arrays
    - String
      - Heap
  - Iteration
    - for loop
    - while loop
    - infinite loop
    - iterators
      - skip?
      - collect
  - flow control
    - if / else
    - if let
    - match
    - \_ as a catchall
  - Using the documentation
  - Type system
    - Primitives
      - str
        - stack
        - `&str`
        - `$'static str`
      - usize
    - Compound Types
      - What is a compound type (consists of multiple other types)
      - enums (with variants)
      - struct
        - methods
    - Creating Custom types
  - module system
    - running functions in another file
      - mod statement
      - `mod command;` (command being command.rs in same file)
      - `mod todo;` (todo being a folder that has a mod.rs in that folder)
    - use statement
      - `use std::fs::{File, OpenOptions};`
      - `use std::io::prelude::*;`
      - `use std::io::BufReader;`
    - re-publishing modules to other files
    - super
  - error handling
    - result
    - unwrapping
    - ? operator (https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html)
  - what about null
    - None, Some
  - what are macros
  - symbols
    - :: (symbol resolution operator)
  - testing
    - tests in the same file
    - tests in doc comments
    - assert
    - assert_eq
  - impl blocks
    - inherit
    - trait
- std library
  - arguments from the command line
  - files
    - writing to files
      - truncate
      - append
    - reading from files
  - useful macros
    - format!
- Cargo
  - compiling
      <!-- - Run -->
    - build
    - configuration - debug - release
    <!-- - new -->
  - test
  - clean
  - docs
  - cargo vs rustc
  - help
  - installing packages
    - ripgrep
  - check
