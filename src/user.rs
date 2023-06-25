use points::Point;
use std::collections::HashMap;

#[derive(Debug)]
struct User {
    id: String,
    points: HashMap<Point, u32>,
}

impl User {
    fn new(id: String) -> User {
        User {
            id: id ,
            points: HashMap::new(),
        }
    }
}