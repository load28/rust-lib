use super::collection;

#[cfg(test)]
pub mod tests {
    use super::collection;

    #[test]
    fn vec() {
        collection::vec::new_vec();

        let array = vec![1, 2, 3];
        let value = match array.get(4) {
            Some(value) => value,
            None() => 30,
        };
    }
}
