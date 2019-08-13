extern crate clip;

use clip::App;
use std::env::args;


fn main() {
    let strings = &args().collect::<Vec<String>>()[..];
    let values: Vec<&str> = strings.iter().map(|s| &**s).collect();



    let mut app = App::new(&values[1..])
                .name("name")
                .flag("--input", 1)
                .flag("--output", 1)
                .flag("--link", -1)
                ;
    app.parse();

    println!(
        "name: {}\ninput: {}\noutput: {}\nlink: {:?}",
        app.get("name")[0],
        app.get("--input")[0],
        app.get("--output")[0],
        app.get("--link"),
    )
}