pub fn smallest_multiple_greater_than(multiple: i32, threshold: i32) -> i32 {
    if threshold % multiple == 0 {
        threshold
    } else {
        (threshold / multiple + 1) * multiple
    }
}
