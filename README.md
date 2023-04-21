# RustLearning

## What is Rust?
[Rust](https://www.rust-lang.org/) is a general-purpose programming language that was developed by Mozilla. It is designed to be fast, safe, and concurrent, making it ideal for systems programming. Rust's syntax is similar to C++ and it provides memory safety without the need for **garbage collection**. Additionally, Rust has a strong focus on preventing common programming errors such as null pointer dereferencing and buffer overflows. Overall, Rust is a modern language that offers high performance and security features while still being easy to learn and use.

Some of the main features of Rust programming language are:

1. Memory safety: Rust has a unique *ownership* and *borrowing* system that ensures memory safety at compile time, preventing common errors such as null pointer dereferencing and buffer overflows.

2. Concurrency: Rust provides built-in support for concurrency through its lightweight threads called "tasks" and message passing between them.

3. Performance: Rust is designed to be fast and efficient, with low-level control over hardware resources like memory allocation and CPU usage.

4. Trait-based generics: Rust's generic types are based on *traits*, which allow for more flexible and reusable code than traditional object-oriented programming languages.

5. Pattern matching: Rust has powerful pattern matching capabilities that make it easy to handle complex data structures in a concise way.

6. Cross-platform compatibility: Rust can be compiled to run on multiple platforms including Windows, Linux, macOS, iOS, Android, WebAssembly, and more.

7. Community-driven development: The development of Rust is driven by an active community of contributors who collaborate on open-source projects and share best practices for writing safe and efficient code.

## Who are using Rust?
There are several notable companies and organizations that use Rust for their projects. Some examples include Mozilla (which created Rust), Dropbox, Microsoft, Amazon Web Services, Google's Fuchsia OS project, Cloudflare, and many more.

## Is Rust easy to learn?
Rust is generally considered to be **a more challenging language** for beginners compared to other programming languages such as Python or JavaScript. This is because Rust has a steeper learning curve due to its focus on memory safety and performance, which requires a deeper understanding of computer architecture and low-level programming concepts. However, with dedication and practice, beginners can certainly learn Rust and benefit from its unique features such as strong type checking, ownership model, and concurrency support. There are also many resources available online such as tutorials, documentation, and community forums that can help beginners get started with Rust.

## What's in this repository?
There are some codes demostrated in the book: [The Rust Programming Language](https://doc.rust-lang.org/book/). Below, I have provided a corresponding list of chapters and codes in the book.

- ch-01: Getting Started
    - Installation
    - [Hello, World!](https://doc.rust-lang.org/book/ch01-02-hello-world.html), [⌨️code](https://github.com/fansuregrin/RustLearning/tree/main/hello_world)
    - [Hello, Cargo!](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html), [⌨️code](https://github.com/fansuregrin/RustLearning/tree/main/hello_cargo)
- ch-02: [Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html), [⌨️code](https://github.com/fansuregrin/RustLearning/tree/main/guesssing_game)
- ch-03: Common Programming Concepts
    - [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html), [⌨️code](https://github.com/fansuregrin/RustLearning/tree/main/variables)
    - Data Types
    - [Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html), [⌨️code](https://github.com/fansuregrin/RustLearning/tree/main/functions)
    - Comments
    - Control Flow
        - [`if` Expressions](https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions), [⌨️code](https://github.com/fansuregrin/RustLearning/tree/main/branches)
        - [Repetition with Loops](https://doc.rust-lang.org/book/ch03-05-control-flow.html#repetition-with-loops), [⌨️code](https://github.com/fansuregrin/RustLearning/tree/main/loops)
- ch-04: [Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html), [⌨️code](https://github.com/fansuregrin/RustLearning/tree/main/ownership)
    - What is Ownership?
    - References and Borrowing
    - [The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html), [⌨️code](https://github.com/fansuregrin/RustLearning/tree/main/slice)
- ch-05: Using Structs to Structure Related Data
    - [Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html), [⌨️code](https://github.com/fansuregrin/RustLearning/tree/main/structs)
    - [An Example Program Using Structs](https://doc.rust-lang.org/book/ch05-02-example-structs.html), [⌨️code](https://github.com/fansuregrin/RustLearning/tree/main/rectangles)
    - [Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html), [⌨️code](https://github.com/fansuregrin/RustLearning/tree/main/rectangles)
- ch-06: [Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html), [⌨️code](https://github.com/fansuregrin/RustLearning/tree/main/enum_examples)
    - Defining an Enum
    - The match Control Flow Construct
    - Concise Control Flow with if let
- ch-07: [Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html), [⌨️code1](https://github.com/fansuregrin/RustLearning/tree/main/backyard), [⌨️code2](https://github.com/fansuregrin/RustLearning/tree/main/restaurant)
    - Packages and Crates
    - Defining Modules to Control Scope and Privacy
    - Paths for Referring to an Item in the Module Tree
    - Bringing Paths Into Scope with the use Keyword
    - Separating Modules into Different Files
- ch-08: Common Collections
    - [Storing Lists of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html), [⌨️code](https://github.com/fansuregrin/RustLearning/tree/main/vector_examples)
    - [Storing UTF-8 Encoded Text with Strings](https://doc.rust-lang.org/book/ch08-02-strings.html), [⌨️code](https://github.com/fansuregrin/RustLearning/tree/main/string_examples)
    - [Storing Keys with Associated Values in Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html), [⌨️code](https://github.com/fansuregrin/RustLearning/tree/main/hashmap_examples)
        - Three Exercises in Summary: [🎯exercise 1](https://github.com/fansuregrin/RustLearning/tree/main/median_and_mode), [🎯exercise 2](https://github.com/fansuregrin/RustLearning/tree/main/pig_latin)
- ch-09: [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html), [⌨️code](https://github.com/fansuregrin/RustLearning/tree/main/error_handling)
    - Unrecoverable Errors with panic!
    - Recoverable Errors with Result
    - To panic! or Not to panic!
- ch-10: Generic Types, Traits, and Lifetimes
    - [Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html), [⌨️code](https://github.com/fansuregrin/RustLearning/tree/main/generic_types)
    - [Traits: Defining Shared Behavior](https://doc.rust-lang.org/book/ch10-02-traits.html), [⌨️code1](https://github.com/fansuregrin/RustLearning/tree/main/trait_examples), [⌨️code2](https://github.com/fansuregrin/RustLearning/tree/main/aggregator)
    - [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html), [⌨️code](https://github.com/fansuregrin/RustLearning/tree/main/lifetime_examples)
- ch-11: [Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html), [⌨️code](https://github.com/fansuregrin/RustLearning/tree/main/auto_test)
    - How to Write Tests
    - Controlling How Tests Are Run
    - Test Organization
- ch-12: [An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html), [⌨️code](https://github.com/fansuregrin/RustLearning/tree/main/minigrep)
    - Accepting Command Line Arguments
    - Reading a File
    - Refactoring to Improve Modularity and Error Handling
    - Developing the Library’s Functionality with Test Driven Development
    - Working with Environment Variables
    - Writing Error Messages to Standard Error Instead of Standard Output
- ch-13: Functional Language Features: Iterators and Closures
    - Closures: Anonymous Functions that Capture Their Environment
        - [Capturing the Environment with Closures](https://doc.rust-lang.org/book/ch13-01-closures.html#capturing-the-environment-with-closures), [⌨️code](https://github.com/fansuregrin/RustLearning/blob/main/shirts_promotion/src/main.rs)