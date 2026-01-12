use pathifier::util::find_root_of_repo;

fn main() {
    match find_root_of_repo() {
        Ok(path) => print!("{}", path.display()),
        Err(_) => std::process::exit(1),
    }
}
