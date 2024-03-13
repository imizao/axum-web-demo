use once_cell::sync::Lazy;

use std::sync::Mutex;

use std::collections::HashMap;

use crate::book::Book;

pub static DATA: Lazy<Mutex<HashMap<u32, Book>>> = Lazy::new(|| {
    Mutex::new(HashMap::from([
        (
            1,
            Book {
                id: 1,
                title: "Antigone".into(),
                author: "Sophocles".into(),
            },
        ),
        (
            2,
            Book {
                id: 2,
                title: "Beloved".into(),
                author: "Toni Morrison".into(),
            },
        ),
        (
            3,
            Book {
                id: 3,
                title: "Candide".into(),
                author: "Voltaire".into(),
            },
        ),
    ]))
});
