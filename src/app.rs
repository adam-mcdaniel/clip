use crate::{Arg, args, Values, NUM_VALUES};


pub const NUM_ARGS: usize = 32;

#[derive(Debug)]
pub struct App<'a> {
    values: Values<'a>,

    num_flags: usize,
    flags: [Arg<'a>; NUM_ARGS],

    num_names: usize,
    names: [Arg<'a>; NUM_ARGS],
}


impl<'a> App<'a> {
    pub fn new(input: &'a [&'a str]) -> Self {
        Self {
            values: args(input),

            num_flags: 0,
            flags: [Arg::new(""); NUM_ARGS],
            
            num_names: 0,
            names: [Arg::new(""); NUM_ARGS],
        }
    }

    pub fn get(&self, name: &'a str) -> Values<'a> {
        for arg in self.names.iter() {
            if arg.has_name(name) { return arg.get_values() }
        }

        for arg in self.flags.iter() {
            if arg.has_name(name) { return arg.get_values() }
        }

        [""; NUM_VALUES]
    }

    pub fn parse(&mut self) {

        'outer: for _ in 0..NUM_ARGS {
            if self.values[0] == "" { break; }

            for flag in self.flags.iter_mut() {
                if flag.consume(&mut self.values) { continue 'outer; }
            }

            for name in self.names.iter_mut() {
                if name.consume(&mut self.values) { continue 'outer; }
            }
        }
    }

    pub fn arg(mut self, name: &'a str, num_values: i32) -> Self {
        let arg = Arg::new(name).num_values(num_values);

        if arg.is_flag() {
            self.push_flag(arg);
        } else {
            self.push_name(arg);
        }

        self
    }

    pub fn flag(self, name: &'a str, num_values: i32) -> Self {
        self.arg(name, num_values)
    }  

    pub fn name(self, name: &'a str) -> Self {
        self.arg(name, 1)
    }

    pub fn push_flag(&mut self, arg: Arg<'a>) {
        self.num_flags += 1;
        self.flags[self.num_flags] = arg;
    }

    pub fn push_name(&mut self, arg: Arg<'a>) {
        self.num_names += 1;
        self.names[self.num_names] = arg;
    }
}