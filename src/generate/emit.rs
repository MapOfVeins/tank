use std::fs::File;
use std::io::{BufWriter, Write};
use std::error::Error;

const LEFT_ANGLE_BRACKET: &'static str = "<";
const RIGHT_ANGLE_BRACKET: &'static str = ">";
const CLOSING_TAG: &'static str = "</";
const EQUALS: &'static str = "=";
const NEWLINE: &'static str = "\n";
const QUOTE: &'static str = "\"";

pub struct Emitter {
    writer: BufWriter<File>
}

impl Emitter {
    pub fn new(w: BufWriter<File>) -> Emitter {
        Emitter {
            writer: w
        }
    }

    /// Writes a string to the current file (determined by which file the BufWriter)
    /// is attached to.
    pub fn emit(&mut self, output: &str) {
        match write!(self.writer, "{}", output) {
            Err(error) => panic!("tank: Failed to write -  {}", Error::description(&error)),
            Ok(_) => ()
        };
    }

    /// Write a '<' symbol to file.
    pub fn left_angle_bracket(&mut self) {
        self.emit(LEFT_ANGLE_BRACKET);
    }

    /// Write a '>' symbol to file.
    pub fn right_angle_bracket(&mut self) {
        self.emit(RIGHT_ANGLE_BRACKET);
    }

    /// Write '=' symbol to file.
    pub fn equals(&mut self) {
        self.emit(EQUALS);
    }

    /// Writes a closing tag to the file for the current element. Requires the element name
    /// to be passed in, and then produces '</el_name>', followed by a new line.
    pub fn close_element(&mut self, tag_value: &str) {
        let mut tag = String::from(CLOSING_TAG);
        tag = tag + tag_value;
        tag = tag + RIGHT_ANGLE_BRACKET;
        tag = tag + NEWLINE;

        self.emit(&tag);
    }

    /// Writes a defined number of spaces to file. Used to provide correct indentation
    /// for nested elements and contents, as well as to separate attributes.
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

    /// Writes the provided value, wrapped in double quotes. Eg. "str_val"
    pub fn string(&mut self, str_val: &str) {
        let val = QUOTE.to_owned() + str_val + QUOTE;

        self.emit(&val);
    }

    /// Writes a new line to file.
    pub fn newline(&mut self) {
        self.emit(NEWLINE);
    }
}
