fn main() {
    let _a = CustomSmartPointer {
        data: String::from("Mondstadt"),
    };
    let _b = CustomSmartPointer {
        data: String::from("Liyue"),
    };
    let c = CustomSmartPointer {
        data: String::from("Inazuma"),
    };
    
    println!("CustomSmartPointers created.");
    // c.drop();  // explicit destructor calls not allowed
    drop(c);  // Calling `std::mem::drop` to explicitly drop a value before it goes out of scope.
    println!("CustomSmartPointer dropped before the end of main.");

    // At the end of main, our instances of `CustomSmartPointer` (i.e., `_a` and `_b`) 
    // will go out of scope, and Rust will call the code we put in the `drop` method,
    // printing some messages.
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}