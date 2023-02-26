use std::collections::HashMap;

fn main() {
    let mut integers = vec![3, 6, 1, 2, 7, 4, 2, 4];
    println!("median: {}", get_median(&mut integers));
    println!("mode: {}", get_mode(&mut integers));
    println!("{:?}", integers);
}

fn get_median(list: &mut Vec<i32>) -> i32 {
    list.sort();
    list[list.len()/2]
}

fn get_mode(list: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut max_count = 0;
    let mut mode = 0;
    for num in list {
        let count = map.entry(num).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            mode = *num;
        }
        
    }
    mode
}
