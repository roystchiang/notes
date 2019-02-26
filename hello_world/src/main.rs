#[macro_use]
extern crate serde_derive;

mod note;

use std::env;

fn main() {
    // parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify a command");
        return ();
    }
    match args[1].as_ref() {
        "load" => {
            let graph = note::list_notes(&args[2]);
            println!("{:?}", graph);

        },
        _ => println!("Unknown command \"{}\"", args[1])
    }
}
