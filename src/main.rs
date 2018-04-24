extern crate tabwriter;
extern crate bloguen;
extern crate url;

use url::percent_encoding::percent_decode;
use std::io::{Write, stderr, stdout};
use std::iter::FromIterator;
use tabwriter::TabWriter;
use std::process::exit;
use std::fs;


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
    println!();

    for p in &posts {
        for link in p.generate(&opts.output_dir)?.into_iter().filter(|l| bloguen::util::is_asset_link(l)) {
            let link = percent_decode(link.as_bytes()).decode_utf8().unwrap();

            let source = link.split('/').fold(p.source_dir.1.clone(), |cur, el| cur.join(el));
            if source.exists() {
                let output = link.split('/').fold(opts.output_dir.1.join("posts"), |cur, el| cur.join(el));

                fs::create_dir_all(output.parent().unwrap()).map_err(|e| {
                        bloguen::Error::Io {
                            desc: "asset parent dir",
                            op: "create",
                            more: Some(e.to_string()),
                        }
                    })?;
                fs::copy(source, output).map_err(|e| {
                        bloguen::Error::Io {
                            desc: "asset",
                            op: "copy",
                            more: Some(e.to_string()),
                        }
                    })?;
            } else {
                eprintln!("Couldn't find \"{}\" for \"{}\" post.", link, p.normalised_name());
            }
        }
    }

    Ok(())
}
