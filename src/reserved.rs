use std::collections::HashMap;

#[derive(Debug)]
pub struct Reserved {
    pub words: HashMap<String, String>
}

// TODO: Better to initialize this hashmap from a constant list (a map is nice
// to have due to constant lookup)
impl Reserved {
    pub fn new() -> Reserved {
        let mut w = HashMap::new();
        w.insert(String::from("if"), String::from("if"));
        w.insert(String::from("let"), String::from("let"));
        w.insert(String::from("for"), String::from("for"));
        w.insert(String::from("in"), String::from("in"));

        // Add types
        w.insert(String::from("int"), String::from("int"));
        w.insert(String::from("bool"), String::from("bool"));
        w.insert(String::from("string"), String::from("string"));

        Reserved {
            words: w
        }
    }
}
