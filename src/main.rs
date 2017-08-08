extern crate bloguen;

use self::bloguen::{Options, Error};
use std::process::exit;
use std::io::stderr;


fn main() {
    let result = actual_main();
    exit(result);
}

fn actual_main() -> i32 {
    if let Err(err) = result_main() {
        err.print_error(&mut stderr());
        err.exit_value()
    } else {
        0
    }
}

fn result_main() -> Result<(), Error> {
    let opts = Options::parse();

    print!("{:#?}", opts);

    Ok(())
}
