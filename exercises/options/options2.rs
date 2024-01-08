// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.



#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target: &str = "rustlings";
        let optional_target: Option<&str> = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        // https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        } 

        // word = optional_target {
        //     assert_eq!(word, target);
        // }
    }

    #[test]
    fn layered_option() {
        let range: i8 = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        // It might seem unusual to see an ﻿Option<Option<T>> in Rust. However, there are situations where it is both valid and useful. The outer ﻿Option could represent if a value exists at all, and the inner ﻿Option could be a breakdown of that value itself.
        // In the case of the code snippet provided, ﻿optional_integers.pop() could be producing an ﻿Option<Option<integer>>. Here, the outer ﻿Option reflects whether the ﻿pop() operation is successful, i.e., if there are elements left in ﻿optional_integers. If there are, ﻿pop() returns ﻿Some(inner_value). If there isn't, it returns ﻿None, ending the loop.
        // The ﻿inner_value is an ﻿Option<integer>, which represents whether a value was stored in the current position of ﻿optional_integers or not. It could be ﻿Some(integer) or ﻿None. The ﻿while let structure is used to destructure this nested Option and allows the loop to continue only if it's ﻿Some(Some(integer)).
        // To draw a Python analogy, it would be like trying to pop from a list in Python, which might bring up an Exception when the list is empty. However, you handle this in the code, letting it to fail quietely returning an equivalent of ﻿None. This inner ﻿None would represent no value in the list, whereas the outer ﻿None would represent no list at all.
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }
        
        assert_eq!(cursor, 0);
    }
}
