use lazy_static::lazy_static;

lazy_static! {
    static ref USERNAME: Regex = {
        println!("Compiling username regex...");
        Regex::new("^[a-z0-9_-]{3,16}$").unwrap()
    };
}

fn main() {
    println!("Let's validate some usernames.");
    validate("fergie");
    validate("will.i.am");
}

fn validate(_name: &str) {
    // The USERNAME regex is compiled lazily the first time its value is accessed.
    //println!("is_match({:?}): {}", name, USERNAME.is_match(name));
}
