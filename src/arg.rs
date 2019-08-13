pub const NUM_VALUES: usize = 16;
pub type Values<'a> = [&'a str; NUM_VALUES];


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum NumArgs {
    None,
    All,
    Number(u32)
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ArgName<'a> {
    Name(&'a str),
    Flag(&'a str)
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Arg<'a> {
    name: ArgName<'a>,
    num_args: NumArgs,
    matched_data: Values<'a>
}


/// Convert slice of strs to args
pub fn args<'a>(a: &[&'a str]) -> Values<'a> {
    let mut result = [""; NUM_VALUES];
    for (i, arg) in a.clone().iter().enumerate() {
        result[i] = arg.clone();
    }
    result
}


fn to_argname<'a>(name: &'a str) -> ArgName<'a> {
    if let Some(ch) = name.chars().nth(0) {
        match ch {
            '-' => {
                ArgName::Flag(name)
            },
            _ => {
                ArgName::Name(name)
            }
        }
    } else {
        ArgName::Name(name)
    }
}

impl<'a> Arg<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name: to_argname(name),
            num_args: NumArgs::Number(1),
            matched_data: [""; NUM_VALUES]
        }
    }

    /// Is this string this argument's name?
    pub fn has_name(&self, name: &'a str) -> bool {
        to_argname(name) == self.name
    }

    /// Is this argument a flag or a name?
    pub fn is_flag(&self) -> bool {
        match self.name {
            ArgName::Name(_) => false,
            ArgName::Flag(_) => true
        }
    }

    /// Set the number of values this argument accepts IF its a flag!
    pub fn num_values(mut self, n: i32) -> Self {
        if let ArgName::Flag(_) = self.name {
            self.num_args = match n {
                -1 => NumArgs::All,
                0 => NumArgs::None,
                n => NumArgs::Number(n as u32),
            };
        }

        self
    }

    pub fn get_values(&self) -> Values<'a> {
        self.matched_data
    }

    /// Test if this Arg could consume a list of values
    /// If this arg has already consumed arguments, it wont match
    /// A name must accept exactly one argument
    pub fn is_match(&self, slice_matches: &[&'a str]) -> bool {
        // Confirm we are an argument
        if self.has_name("") { return false; }

        // Convert slice to array that we can mess with
        let matches = args(slice_matches);

        // Confirm we havent already consumed data
        if self.matched_data[0] != "" { return false; }

        // Get the number of args passed in the slice
        let mut num_args = 0;
        for m in matches.iter() {
            if m.clone() != "" { num_args += 1; }
        }

        // Match the input data based on whether 
        // or not this instance is a flag or a name
        match self.name {
            ArgName::Name(_) => {
                let valid_num_args = match self.num_args {
                    // A name can only have one argument
                    NumArgs::Number(1) => num_args == 1,
                    // A name cant have more than one argument, or none arguments
                    NumArgs::All | NumArgs::Number(_) | NumArgs::None => false,
                };

                valid_num_args
            },
            ArgName::Flag(n) => {
                let valid_num_args = match self.num_args {
                    NumArgs::None => num_args == 1,
                    NumArgs::All => true,
                    NumArgs::Number(n) => num_args == n + 1
                };

                valid_num_args && (matches[0] == n)
            }
        }
    }

    /// Takes an array of values and consumes data from them
    /// Returns true if did consume, returns false if did not consume
    pub fn consume<'b>(&mut self, values: &'b mut Values<'a>) -> bool {
        let mut matched_up_to = 0;
        let mut consumed = false;

        for n in (1..values.len()).rev() {
            if (*values)[n-1] == "" {
                continue;
            }

            if self.is_match(&values[0..n]) {

                if self.is_flag() {
                    self.matched_data = args(&values[1..n]);
                } else {
                    self.matched_data = args(&values[0..n]);
                }
                matched_up_to = n;

                consumed = true;
            }
        }

        *values = args(&values[matched_up_to..]);
        consumed
    }
}