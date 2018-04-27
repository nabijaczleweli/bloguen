extern crate tabwriter;
extern crate bloguen;
extern crate url;

use url::percent_encoding::percent_decode;
use std::io::{Write, stderr, stdout};
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

fn result_main() -> Result<(), bloguen::Error> {
    let opts = bloguen::Options::parse();

    let descriptor = bloguen::ops::BlogueDescriptor::read(&opts.source_dir)?;
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
    println!();

    let post_header = bloguen::util::read_file(&descriptor.header_file, "post header")?;
    let post_footer = bloguen::util::read_file(&descriptor.footer_file, "post footer")?;

    println!("{}", post_header);
    println!("{}", post_footer);

    for p in &posts {
        for link in p.generate(&opts.output_dir)?.into_iter().filter(|l| bloguen::util::is_asset_link(l)) {
            let link = percent_decode(link.as_bytes()).decode_utf8().unwrap();

            if !p.copy_asset(&opts.output_dir, &link)? {
                eprintln!("Couldn't find \"{}\" for \"{}\" post.", link, p.normalised_name());
            }
        }
    }

    Ok(())
}
