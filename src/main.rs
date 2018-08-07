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

    let mut descriptor = bloguen::ops::BlogueDescriptor::read(&opts.source_dir)?;
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

    let mut post_header = bloguen::util::read_file(&descriptor.header_file, "post header")?;
    let mut post_footer = bloguen::util::read_file(&descriptor.footer_file, "post footer")?;
    let global_language = descriptor.language.take().unwrap_or_else(|| match bloguen::util::default_language() {
        Some(l) => {
            match l.parse() {
                Ok(l) => l,
                Err(_) => {
                    eprintln!("Detected system language {} not BCP-47. Defaulting to \"en-GB\".", l);
                    bloguen::util::LANGUAGE_EN_GB.clone()
                }
            }
        }
        None => {
            eprintln!("Couldn't detect system language. Defaulting to \"en-GB\".");
            bloguen::util::LANGUAGE_EN_GB.clone()
        }
    });
    let global_author = descriptor.author.take().unwrap_or_else(|| match bloguen::util::current_username() {
        Some(l) => l,
        None => {
            let uname = bloguen::util::name_based_full_name(&descriptor.name);
            eprintln!("Couldn't detect system username. Generated \"{}\".", uname);
            uname
        }
    });

    bloguen::util::newline_pad(&mut post_header, 0, 2);
    bloguen::util::newline_pad(&mut post_footer, 2, 1);

    println!("{}", post_header);
    println!("{}", post_footer);
    println!("{}", global_language);
    println!("{}", global_author);

    for p in &posts {
        let mut metadata = bloguen::ops::PostMetadata::read_or_default(&p.source_dir)?;
        let language = metadata.language.as_ref().unwrap_or(&global_language);
        let author = metadata.author.as_ref().unwrap_or(&global_author);

        for link in p.generate(&opts.output_dir,
                      &post_header,
                      &post_footer,
                      &descriptor.name,
                      &language,
                      author,
                      &metadata.data,
                      &descriptor.data,
                      &metadata.styles,
                      &descriptor.styles,
                      &metadata.scripts,
                      &descriptor.scripts)?
            .into_iter()
            .filter(|l| bloguen::util::is_asset_link(l)) {
            let link = percent_decode(link.as_bytes()).decode_utf8().unwrap();

            if !p.copy_asset(&opts.output_dir, &link)? {
                eprintln!("Couldn't find \"{}\" for \"{}\" post.", link, p.normalised_name());
            }
        }
    }

    Ok(())
}
