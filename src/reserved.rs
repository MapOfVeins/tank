use std::collections::HashMap;

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

        Reserved {
            words: w
        }
    }
}
