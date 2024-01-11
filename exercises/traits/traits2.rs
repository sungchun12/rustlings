// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.



trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
// 1. This code implements the trait "AppendBar" for the vector type "Vec<String>".
// 2. The "append_bar" function is defined, which takes in a mutable reference to the vector and returns a mutable reference to the same vector.
// 3. Within the function, the "push" method is used to add the string "Bar" to the end of the vector.
// 4. Finally, the updated vector is returned.
// 5. This code is not broken and is a valid implementation of the AppendBar trait for the Vec<String> type.
// 6. The "mut" keyword indicates that the vector is mutable and can be modified within the function.
// 7. The "self" keyword refers to the vector itself, and the "push" method adds the specified string to the end of the vector.
// 8. By returning a mutable reference to the vector, the changes made within the function are reflected outside of it as well.
// 9. This allows the user to call the "append_bar" function on a vector and have the string "Bar" automatically added to the end of it.
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar"));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
