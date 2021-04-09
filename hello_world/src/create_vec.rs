pub enum Multiple {
    Integer(i32),
    Blob(String),
}

impl std::fmt::Display for Multiple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Multiple::Integer(v) => v.fmt(f),
            Multiple::Blob(v) => v.fmt(f),
        }
    }
}

// #[test]

// fn test_create_vector() {
//     let v = create_vector();
//     let integer = Multiple::Integer(v);
// }

pub fn create_vector() -> Vec<Multiple> {
    let v2 = vec![
        Multiple::Integer(100),
        Multiple::Blob("Hello World".to_string()),
    ];
    v2
}
