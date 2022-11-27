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
        let _value = match array.get(4) {
            Some(value) => value,
            None => &30,
        };
    }

    #[test]
    fn enum_vec() {
        #[derive(Debug)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];

        enum User {
            Admin(String),
            User(i32),
        }

        let users = vec![
            User::Admin(String::from("admin")),
            User::User(1),
        ];

        let _none = &String::from("NONE");
        let user_value = match users.get(1) {
            Some(value) => match value {
                User::Admin(x) => x,
                _ => _none,
            },
            None => _none,
        };

        if user_value != _none {
            println!("user_value: {}", user_value);
        }

        let row = row.get(1);
        let value = match row {
            Some(value) => {
                if let SpreadsheetCell::Int(value2) = value {
                    value2
                } else {
                    &0
                }
            },
            None => &0,
        };

        println!("row: {:?}", value);
    }
}
