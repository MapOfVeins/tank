use std::collections::HashMap;
use token::TokenType;

#[derive(Debug)]
pub struct Reserved {
    pub words: HashMap<String, u16>
}

// TODO: Better to initialize this hashmap from a constant list (a map is nice
// to have due to constant lookup)
impl Reserved {
    pub fn new() -> Reserved {
        let mut w = HashMap::new();
        w.insert(String::from("if"), 0);
        w.insert(String::from("let"), 1);
        w.insert(String::from("for"), 2);
        w.insert(String::from("in"), 3);

        // Add types
        w.insert(String::from("int"), 4);
        w.insert(String::from("bool"), 5);
        w.insert(String::from("string"), 6);

        Reserved {
            words: w
        }
    }
}
