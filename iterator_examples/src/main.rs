fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    // creating an iterator
    let v1_iter = v1.iter();
    // using an iterator in a `for` loop
    for val in v1_iter {
        println!("Got: {val}");
    }

    let v2 = vec![1, 2, 3];
    let mut v2_iter = v2.iter();
    // calling the `next` method on an iterator
    assert_eq!(v2_iter.next(), Some(&1));
    assert_eq!(v2_iter.next(), Some(&2));
    assert_eq!(v2_iter.next(), Some(&3));
    assert_eq!(v2_iter.next(), None);
    // We didn’t need to make `v2_iter` mutable when we used a `for` loop
    // because the loop took ownership of `v2_iter` and made it mutable behind the scenes.
    
    // The `iter` method produces an iterator over **immutable references**.
    // Using the `into_iter` method if we want to create an iterator that
    // takes ownership of *a series of items* and returns owned values.
    // Using the `iter_mut` method if we want to iterate over **mutable references**.

    let mut v3 = vec![1, 2, 3];
    let v3_iter = v3.iter_mut();
    for val in v3_iter {
        *val = *val + 1;
    }
    println!("v3: {:?}", v3);

}

#[cfg(test)]
mod test {
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 3, 4];
        let v1_iter = v1.iter();
        // Calling the sum method to get the total of all items in the iterator
        let total: i32 = v1_iter.sum();
        assert_eq!(8, total);
        
        // We aren’t allowed to use `v1_iter` after the call to `sum` because
        // `sum` takes ownership of the iterator we call it on.
        // for val in v1_iter {
        //     println!("Got: {val}");
        // }
    }
}
