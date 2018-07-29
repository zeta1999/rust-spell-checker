use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::mem;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("In file {}", config.filename);

    // Reading in a file
    let mut f = File::open(config.filename).expect("File Not Found :(");

    // let path = std::path::Path::new(&config.filename);
    // let file = std::fs::File::create(&path).unwrap();

    // let mut zip = zip::ZipWriter::new(file);

    // try!(zip.add_directory("test/", FileOptions::default()));

    // let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored).unix_permissions(0o755);
    // try!(zip.start_file("test/☃.txt", options));
    // try!(zip.write_all(b"Hello, World!\n"));

    // try!(zip.start_file("test/lorem_ipsum.txt", FileOptions::default()));

    // try!(zip.finish());
    
    let mut contents = String::new();
    &f.read_to_string(&mut contents)
    .expect("Something went wrong :( Could not read the file");

    let mut word_file_contents = String::new();
    let mut word_file = File::open("words.txt").expect("File Not Found :(");
    &word_file.read_to_string(&mut word_file_contents)
    .expect("Something went wrong :( Could not read the file");


    // println!("Contains:\n{}", contents);
}



struct Config {
    filename: String,
    query : String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.\nYou must specify a filename to check.");
        }
        else if args.len() > 3 {
            return Err("Can only check one file at a time.")
        }
        let filename = args[2].clone();
        let query = args[1].clone();

        Ok(Config {query, filename})
    }
}