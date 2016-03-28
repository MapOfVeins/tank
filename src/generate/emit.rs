use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::error::Error;

const LEFT_ANGLE_BRACKET: &'static str = "<";
const RIGHT_ANGLE_BRACKET: &'static str = ">";
const CLOSING_TAG: &'static str = "</";
const EQUALS: &'static str = "=";
const NEWLINE: &'static str = "\n";

pub struct Emitter {
    writer: BufWriter<File>
}

impl Emitter {
    pub fn new(w: BufWriter<File>) -> Emitter {
        Emitter {
            writer: w
        }
    }

    pub fn emit(&mut self, output: &String) {
        match write!(self.writer, "{}", output) {
            Err(error) => panic!("tank: Failed to write -  {}", Error::description(&error)),
            Ok(_) => ()
        };
    }

    pub fn left_angle_bracket(&mut self) {
        self.emit(&LEFT_ANGLE_BRACKET.to_owned());
    }

    pub fn right_angle_bracket(&mut self) {
        self.emit(&RIGHT_ANGLE_BRACKET.to_owned());
    }

    pub fn equals(&mut self) {
        self.emit(&EQUALS.to_owned());
    }

    pub fn close_element(&mut self, tag_value: &String) {
        let mut tag = String::from(CLOSING_TAG);
        tag = tag + tag_value;
        tag = tag + RIGHT_ANGLE_BRACKET;
        tag = tag + NEWLINE;

        self.emit(&tag);
    }

    pub fn space(&mut self, count: usize) {
        if count == 0 {
            return;
        }

        let mut spaces = String::from("");
        let mut i = 0;
        while i < count {
            spaces = spaces + " ";
            i = i + 1;
        }

        self.emit(&spaces);
    }

    pub fn string(&mut self, str_val: &String) {
        let mut val = "\"".to_owned();
        val = val + str_val;
        val = val + "\"";

        self.emit(&val);
    }

    pub fn newline(&mut self) {
        self.emit(&NEWLINE.to_owned());
    }
}
