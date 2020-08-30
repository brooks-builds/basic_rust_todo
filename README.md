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
