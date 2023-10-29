mod lib;

fn main() {
    if let Err(err) = lib::get_args().and_then(lib::run){
        eprint!("Error: {}", err);
        std::process::exit(1);
    }

}

