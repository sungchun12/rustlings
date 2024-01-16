// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.



pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

// The code example you provided utilizes dynamic dispatch and polymorphism, thanks to the trait system in Rust. In comparison to ﻿match statements, this design brings a number of advantages:
// 	1.	Flexibility: It allows different types to be treated uniformly. Allocating different actions in ﻿match statements can start to become difficult to manage as the number of possible types increases.
// 	2.	Scalability: The trait system easily scales with the code base. If a new type is added and an implementation of ﻿Grade is provided, no changes are needed elsewhere in the code. Conversely, using ﻿match would require updating all places where grade is evaluated.
// 	3.	Code Organization and Readability: Trait implementations encapsulate functionality related to a specific data type in one place. This modularizes the code better than ﻿match-based approaches which can scatter type-specific implementations across the code base.
// 	4.	Enforces Single Responsibility Principle: Each implementation block addresses the needs of a single type. This ensures that a given function or method is modified only for reasons relevant to its assigned type. In contrast, modification of a ﻿match statement affects all branch possibilities, which can lead to unintended side effects.
// 	5.	Code Reuse: Implementing a method for a trait means it's usable for all types that implement this trait. By contrast, ﻿match statements would require case-by-case implementations.
// Note that there could be some performance overhead of dynamic dispatch over static dispatch (﻿match or ﻿if-else statements that get resolved at compile-time). But in majority of applications, the benefits of modularity, maintainability, and code clarity are considered more important.
// Create a trait named Grade that represents a generic grade type and implement for the struct ReportCard.
pub trait Grade<T> {
    fn grade(&self) -> T;
}

// creates a function specific to the type of grade which is an integer
impl Grade<f32> for ReportCard<f32> {
    fn grade(&self) -> f32 {
        self.grade
    }
}

// creates a function specific to the type of grade which is a string
impl Grade<String> for ReportCard<String> {
    fn grade(&self) -> String {
        self.grade.clone()
    }
}

// This function dynamically dispatches the correct function based on the type of grade implemented by the trait functions above. Makes it more effective than a match statement.
impl<T: std::fmt::Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: "A+".to_string(),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
