pub mod arg;

fn main() {
    let cli = arg::arg();
    let src = match std::fs::read_to_string(cli.bffile) {
        Ok(s) => s,
        Err(e) => {
            panic!("{}", e);
        }
    };
    let result = bf_core::run(&src);
    match result {
        Ok(s) => {
            print!("{}", s);
        }
        Err(e) => {
            eprint!("{}", e);
        }
    }
}
