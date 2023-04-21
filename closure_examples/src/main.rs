use std::thread;
use std::time;

fn main() {
    // ====================================
    // Closure Type Inference and Anotation
    // ====================================

    // Closures don’t usually require you to annotate the types of the parameters
    // or the return value like fn functions do.
    // Type annotations are required on functions because the types are part of
    // an explicit interface exposed to your users.
    // As with variables, we can add type annotations if we want to increase explicitness
    // and clarity at the cost of being more verbose than is strictly necessary.
    //
    let _expensive_closure = |num: u64| -> u64 {
        println!("calculating slowly...");
        thread::sleep(time::Duration::from_secs(num));
        num
    }; 

    // The _add_one_v3 and _add_one_v4 lines require the closures to be evaluated
    // to be able to compile because the types will be inferred from their usage.
    //
    // fn  _add_one_v1   (x: u32) -> u32 { x + 1 }
    // let _add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let _add_one_v3 = |x|             { x + 1 };
    // let _add_one_v4 = |x|               x + 1  ;

    let example_closure = |x| x;
    let _s = example_closure(String::from("hi"));

    // This line won't be compiled: `error[E0308]: mismatched types`,
    // expected struct `String`, found integer.
    // The first time we call example_closure with the String value,
    // the compiler infers the type of x and the return type of the closure to be String.
    // Those types are then locked into the closure in example_closure,
    // and we get a type error when we next try to use a different type with the same closure.
    //
    // let _n = example_closure(1);

    // ========================================
    // Capturing References or Moving Ownership
    // ========================================

    // Closures can capture values from their environment in **three** ways,
    // which directly map to the three ways a function can take a parameter:
    // **borrowing immutably**, **borrowing mutably**, and **taking ownership**.

    // A Closure Capturing an Immutable Reference
    let list_a = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list_a);
    
    // Below, we define a closure that captures an immutable reference to
    // the vector named `list_a` because it only needs an immutable reference to print the value.
    let only_borrows = || println!("From closure: {:?}", list_a);

    println!("Before calling closure: {:?}", list_a);
    only_borrows();
    println!("After calling closure: {:?}", list_a);

    // A Closure Capturing an Mutable Reference
    let mut list_b = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list_b);

    // In the body of the closure, we add a new element to `list_b`.
    let mut borrow_mutably = || list_b.push(4);

    // This line will cause a compiling error,
    // because no other borrows are allowed when there’s a mutable borrow
    // println!("Before calling closure: {:?}", list_b);

    borrow_mutably();
    println!("After calling closure: {:?}", list_b);

    // A Closure Moving Ownership
    let list_c = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list_c);

    // If you want to force the closure to take ownership of the values it uses
    // in the environment even though the body of the closure doesn’t strictly need ownership,
    // you can use the `move` keyword before the parameter list.
    // If you delete the `move` keyword, there will be an error:
    // "closure may outlive the current function, but it borrows `list_c`, which is owned by the current function."
    thread::spawn(move || println!("From thread: {:?}", list_c))
        .join()
        .unwrap();

    // This line will raise an error: "borrow of moved value: `list_c`."
    // println!("After calling closure: {:?}", list_c);

    // ========================================================
    // Moving Captured Values Out of Closures and the Fn Traits
    // ========================================================

    // Closures will **automatically** implement one, two, or all three of these `Fn` traits,
    // in an **additive** fashion, depending on how the closure’s body handles the values.
    // A closure body can do any of the following:
    //      - move a captured value out of the closure,
    //      - mutate the captured value,
    //      - neither move nor mutate the value,
    //      - capture nothing from the environment to begin with.
    //
    // 1. `FnOnce` applies to closures that can be called once.
    //    All closures implement at least this trait, because all closures can be called.
    //    A closure that **moves captured values out of its body** will only implement `FnOnce`
    //    and none of the other `Fn` traits, because it can only be called once.
    // 2. `FnMut` applies to closures that **don’t move captured values out of their body**,
    //    but that might mutate the captured values. These closures can be called more than once.
    // 3. `Fn` applies to closures that **don’t move captured values out of their body**
    //    and that **don’t mutate captured values**, as well as closures that capture nothing
    //    from their environment. These closures can be called more than once without mutating
    //    their environment, which is important in cases such as calling a closure multiple times **concurrently**.

    let mut rect_list = [
        Rectangle { width: 10, height: 1},
        Rectangle { width: 3, height: 5},
        Rectangle { width: 7, height: 12}
    ];
    rect_list.sort_by_key(|r| r.width);
    println!("{:#?}", rect_list);

    // let mut sort_operations: Vec<String> = vec![];
    // let value = String::from("by key called"); 
    // rect_list.sort_by_key(|r| {
    //     // This line will cause an error.
    //     // The closure captures value then moves value out of the closure
    //     // by transferring ownership of value to the `sort_operations` vector.
    //     // Therefore, this closure only implements `FnOnce`, but `sort_by_key` takes
    //     // a parameter that implemented `FnMut` trait.
    //     //
    //     sort_operations.push(value);
        
    //     r.height
    // });
    // println!("{:#?}", rect_list);

    let mut num_sort_operations = 0;
    rect_list.sort_by_key(|r| {
        num_sort_operations += 1; 
        r.height
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", rect_list);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}