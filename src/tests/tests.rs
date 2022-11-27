use super::super::collection::{vec};

#[cfg(test)]
pub mod collection_test {
    use super::vec::{new_vec};

    #[test]
    fn vec_fn() {
        new_vec();

        let array = vec![1, 2, 3];
        let value = match array.get(4) {
            Some(value) => value,
            None => &30,
        };
    }
}

#[cfg(test)]
pub mod array_test {
    #[test]
    fn array() {
        let array = [1, 2, 3];
        let value = match array.get(4) {
            Some(value) => value,
            None => &30,
        };
    }
}
