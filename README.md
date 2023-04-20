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