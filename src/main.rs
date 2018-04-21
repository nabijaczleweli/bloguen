extern crate tabwriter;
extern crate bloguen;

use std::io::{Write, stderr, stdout};
use self::bloguen::{Options, Error};
use std::iter::FromIterator;
use tabwriter::TabWriter;
use std::process::exit;


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

    let descriptor = bloguen::ops::BlogueDescriptor::read(&opts.source_dir.1.join("blogue.toml"))?;
    println!("Blog name: {}", descriptor.name);

    let mut posts: Vec<_> = Result::from_iter(bloguen::ops::BloguePost::list(&opts.source_dir)?.into_iter().map(bloguen::ops::BloguePost::new))?;
    posts.sort_by(|lhs, rhs| lhs.number.cmp(&rhs.number).then_with(|| lhs.datetime.cmp(&rhs.datetime)).then_with(|| lhs.name.cmp(&rhs.name)));
    println!("Found {} posts:", posts.len());
    {
        let mut out = TabWriter::new(stdout()).minwidth(1).padding(3);
        for p in &posts {
            writeln!(out, "\t{}\t{}\t{}", p.number.0, p.datetime.format("%Y.%m.%d %r"), p.name).unwrap();
        }
        out.flush().unwrap();
    }

    for p in &posts {
        p.generate(&opts.output_dir)?;
    }

    Ok(())
}
