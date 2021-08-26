use std::cmp::Ordering;

pub fn find<T: PartialOrd, V: AsRef<[T]>>(array: V, key: T) -> Option<usize> {
    let mut right = array.as_ref().len();
    let mut left = 0;
    while left < right {
        let mid = (right + left) / 2;
        match array.as_ref()[mid].partial_cmp(&key)? {
            Ordering::Equal => return Some(mid),
            Ordering::Less => {left = mid + 1},
            Ordering::Greater => {right = mid},
        }
    }
    return None
}
