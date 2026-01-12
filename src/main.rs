use std::{
    collections::HashSet,
    env,
};

fn main() {
    // accept a string
    let args : Vec<_> = env::args().collect();
    let mut set : HashSet<String> = HashSet::new();

    // split string on path delimiters
    let delimiter = if cfg!(unix) {
        ":"
    } else {
        ";"
    };

    let line = args[1].clone();
    let parts : Vec<&str> = line.split(delimiter).collect();
    let mut new_path : Vec<String> = vec![];

    // uniquify string
    for p in parts {
        let is_new = set.insert(p.to_string());
        if is_new {
            new_path.push(p.into());
        }
    }

    // print out results
    let new_path_string = new_path.join(delimiter);
    println!("{}", new_path_string);
}
