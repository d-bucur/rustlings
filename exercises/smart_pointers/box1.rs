// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This becomes problematic
// for recursive types, where a value can have as part of itself another value of the same type.
// To get around the issue, we can use a `Box` - a smart pointer used to store data on the heap,
// which also allows us to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a data structure
// frequently found in functional programming languages. Each item in a cons list contains two
// elements: the value of the current item and the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `todo!()`
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.

#[derive(PartialEq, Debug)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T> List<T> where T : Copy {
    fn new(elems: &[T]) -> List<T> {
        if elems.is_empty() {
            return List::Nil
        }
        let mut start = List::Nil;
        let mut current = &mut start;
        for el in elems {
            *current = List::Cons(*el, Box::new(List::Nil));
            if let List::Cons(_, t) = current {
                current = t.as_mut()
            }
        }
        start
    }
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List<i32> {
    List::Nil
}

pub fn create_non_empty_list() -> List<i32> {
    List::new(&[1, 2, 3])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
