# clip

A command line parser that doesn't use std or alloc!

## Why?

I mainly did this for educational purposes. I've never written a library that did not use the `std` or `alloc` crates, so I wanted to learn how.

I also think the portability of this crate will be useful in implementing [Rusty-CI](https://github.com/adam-mcdaniel/rusty-ci)


## Example

```rust
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
```