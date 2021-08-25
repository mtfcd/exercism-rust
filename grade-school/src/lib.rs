use std::collections::{BTreeMap, BTreeSet};

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School(BTreeMap<u32, BTreeSet<String>>);

impl School {
    pub fn new() -> School {
        Self(BTreeMap::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        match self.0.get_mut(&grade) {
            Some(students) => {students.insert(student.to_owned());},
            None => {
                let mut students = BTreeSet::new();
                students.insert(student.to_owned());
                self.0.insert(grade, students);
            }
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.0.keys().cloned().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.0.get(&grade) {
            Some(students) => students.iter().cloned().collect(),
            None => vec![]
        }
    }
}
