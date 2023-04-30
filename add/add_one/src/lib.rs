pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use rand;
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }

    #[test]
    fn add_one_to_random_num() {
        let rnd_num = rand::random::<i32>();
        assert_eq!(rnd_num + 1, add_one(rnd_num));
    }
}
