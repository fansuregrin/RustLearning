//! # Doc Comments Demo
//! 
//! `doc_comments_demo` is a collection of examples to demostrate the use of
//! documentaion comments in rust programing language.

/// Adds one to the number given.
/// 
/// # Examples
/// 
/// ```
/// let arg = 2;
/// let answer = doc_comments_demo::add_one(arg);
/// 
/// assert_eq!(3, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}