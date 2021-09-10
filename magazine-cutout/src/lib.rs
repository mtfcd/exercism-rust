// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut dict = HashMap::new();
    for s in magazine {
        let mut entry = dict.entry(s).or_insert(0);
        *entry += 1;
    }

    for s in note {
        match dict.get_mut(s) {
            Some(c) => {
                *c -= 1;
                if *c < 0 {
                    return false
                }
            },
            None => return false,
        }
    }

    return true
}
