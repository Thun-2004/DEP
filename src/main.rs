mod lib;
use lib::get_args;
fn main() {
    if let Err(err) = get_args(){
        eprint!("Error: {}", err);
        std::process::exit(1);
    }
}
