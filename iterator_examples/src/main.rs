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

    // Methods that produce other iterators
    // *Iterator adaptors* are methods defined on the Iterator trait that
    // don’t consume the iterator. Instead, they produce different iterators
    // by changing some aspect of the original iterator. 
    let v4 = vec![3, 4, 5];
    // The `map` method returns a new iterator that produces the modified items.
    // The `collect` method consumes the iterator and collects the resulting values into a collection data type.
    let v5: Vec<_> = v4.iter().map(|x| x+1).collect();
    assert_eq!(v5, vec![4, 5, 6]);
    println!("v4: {:?}", v4);
    println!("v5: {:?}", v5);
    // We can **chain multiple calls to iterator adaptors** to perform complex actions in a readable way.
    // But because all iterators are **lazy**, you have to call **one of the consuming adaptor methods**
    // to get results from calls to iterator adaptors.
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

#[allow(unused)]
// The `filter` method takes a `closure` that captures the `shoe_size` variable from its environment
// to iterate over a collection of  `Shoe` struct instances. 
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);
        
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }
}
