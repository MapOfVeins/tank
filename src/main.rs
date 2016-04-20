extern crate tank;

use std::env;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::error::Error;

use tank::compile::compiler::Compiler;

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| {
        panic!("tank: Expected a file or directory name as the first arg.");
    });

    let config_filename = env::args().nth(2);

    let path = Path::new(&filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(error) => panic!("Failed to open {}: {}", display, Error::description(&error)),
        Ok(file) => file
    };

    let metadata = match file.metadata() {
        Ok(data) => data,
        Err(error) => panic!("Failed to get metadata for {}: {}",
                             &filename,
                             Error::description(&error))
    };

    if metadata.is_dir() {
        let files = match fs::read_dir(&filename) {
            Err(error) => panic!("Failed to read directory {}: {}",
                                 &filename,
                                 Error::description(&error)),
            Ok(list) => list
        };

        for entry in files {
            let entry = match entry {
                Ok(e) => e,
                Err(error) => panic!("Failed to read file entry: {}", Error::description(&error))

            };

            let path = entry.path();
            let mut file = match File::open(&path) {
                Err(error) => panic!("Failed to open {}: {}", display, Error::description(&error)),
                Ok(file) => file
            };

            let file_name = path.to_str().unwrap().to_owned();

            // TODO: Add conf file if necessary here
            let mut compiler = Compiler::new(&mut file, &file_name);
            compiler.compile();
        }
    } else {
        if config_filename.is_some() {
            let conf_name = config_filename.unwrap();

            let conf_path = Path::new(&conf_name);
            let mut conf_file = match File::open(&conf_path) {
                Err(error) => panic!("Failed to open {}: {}",
                                     conf_path.display(),
                                     Error::description(&error)),
                Ok(file) => file
            };

            let mut compiler = Compiler::from_config_file(&mut file, &filename, &mut conf_file);
            compiler.compile();
        } else {
            let mut compiler = Compiler::new(&mut file, &filename);
            compiler.compile();
        }
    }
}
