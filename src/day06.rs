use std::cmp::Ordering;

// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    let s1_len = s1.trim().chars().count();
    let s2_len = s2.trim().chars().count();
    match s1_len.cmp(&s2_len) {
        Ordering::Less => Some(s2),
        Ordering::Greater => Some(s1),
        Ordering::Equal => None,
    }
}
