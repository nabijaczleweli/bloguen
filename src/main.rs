extern crate percent_encoding;
extern crate tabwriter;
extern crate bloguen;
extern crate chrono;
extern crate rayon;
extern crate url;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::sync::mpsc::channel as mpsc_channel;
use percent_encoding::percent_decode;
use std::collections::BTreeMap;
use std::io::{Write, stdout};
use std::iter::FromIterator;
use tabwriter::TabWriter;
use std::process::exit;
use std::mem::swap;
use std::fs::File;
use chrono::Utc;


fn main() {
    let result = actual_main();
    exit(result);
}

fn actual_main() -> i32 {
    if let Err(err) = result_main() {
        eprintln!("{}", err);
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
    let (mut index_header, mut index_center, mut index_footer) = if let Some(ref idx) = descriptor.index.as_ref() {
        (Some(bloguen::util::read_file(&idx.header_file, "index header")?),
         Some(bloguen::util::read_file(&idx.center_file, "index center")?),
         Some(bloguen::util::read_file(&idx.footer_file, "index footer")?))
    } else {
        (None, None, None)
    };
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
    if let Some(ref mut index_header) = index_header.as_mut() {
        bloguen::util::newline_pad(index_header, 0, 2);
    }
    if let Some(ref mut index_center) = index_center.as_mut() {
        bloguen::util::newline_pad(index_center, 1, 1);
    }
    if let Some(ref mut index_footer) = index_footer.as_mut() {
        bloguen::util::newline_pad(index_footer, 2, 1);
    }

    for s in &mut descriptor.styles {
        s.load(&opts.source_dir)?;
    }

    for s in &mut descriptor.scripts {
        s.load(&opts.source_dir)?;
    }

    if let Some(idx) = descriptor.index.as_mut() {
        for s in &mut idx.styles {
            s.load(&opts.source_dir)?;
        }

        for s in &mut idx.scripts {
            s.load(&opts.source_dir)?;
        }
    }

    // println!("{}", post_header);
    // println!("{}", post_footer);
    // println!("{}", global_language);
    // println!("{}", global_author);
    // println!("{:#?}", descriptor);


    let mut feed_files: BTreeMap<_, _> =
        Result::from_iter(descriptor.feeds.iter().map(|(tp, fname)| descriptor.create_feed_output(&opts.output_dir, fname, tp).map(|f| (*tp, f))))?;
    for (tp, ff) in &mut feed_files {
        descriptor.generate_feed_head(ff, tp, &global_language, &global_author);
    }


    let (idx_sender, idx_receiver) = mpsc_channel();

    posts.par_iter()
        .try_for_each_with(idx_sender, |idx_sender, p| {
            let mut metadata = bloguen::ops::PostMetadata::read_or_default(&p.source_dir)?;
            let language = metadata.language.as_ref().unwrap_or(&global_language);
            let author = metadata.author.as_ref().unwrap_or(&global_author);

            for s in &mut metadata.styles {
                s.load(&p.source_dir)?;
            }

            for s in &mut metadata.scripts {
                s.load(&p.source_dir)?;
            }

            let independent_tags = bloguen::ops::TagName::load_additional_post_tags(&p.source_dir)?;

            let mut index_machine_json = vec![];
            for (kind, subpath) in &descriptor.machine_data {
                let mut f_out = p.create_machine_output(&opts.output_dir, subpath, kind)?;

                if *kind == bloguen::ops::MachineDataKind::Json && !index_machine_json.is_empty() && descriptor.index.is_some() {
                    p.generate_machine(&mut bloguen::util::PolyWrite(f_out, &mut index_machine_json),
                                          kind,
                                          &descriptor.name,
                                          &language,
                                          author,
                                          &metadata.tags,
                                          &independent_tags,
                                          &metadata.data,
                                          &descriptor.data,
                                          &metadata.styles,
                                          &descriptor.styles,
                                          &metadata.scripts,
                                          &descriptor.scripts)?;
                } else {
                    p.generate_machine(&mut f_out,
                                          kind,
                                          &descriptor.name,
                                          &language,
                                          author,
                                          &metadata.tags,
                                          &independent_tags,
                                          &metadata.data,
                                          &descriptor.data,
                                          &metadata.styles,
                                          &descriptor.styles,
                                          &metadata.scripts,
                                          &descriptor.scripts)?;
                }
            }

            if descriptor.index.is_some() && index_machine_json.is_empty() {
                p.generate_machine(&mut index_machine_json,
                                      &bloguen::ops::MachineDataKind::Json,
                                      &descriptor.name,
                                      &language,
                                      author,
                                      &metadata.tags,
                                      &independent_tags,
                                      &metadata.data,
                                      &descriptor.data,
                                      &metadata.styles,
                                      &descriptor.styles,
                                      &metadata.scripts,
                                      &descriptor.scripts)?;
            }

            let mut center_buffer = vec![];
            for link in p.generate(&opts.output_dir,
                          None,
                          index_center.as_ref().map(|ic| (&ic[..], &mut center_buffer as &mut dyn Write)),
                          descriptor.asset_dir_override.as_ref().map(|s| &s[..]),
                          &post_header,
                          &post_footer,
                          &descriptor.name,
                          &language,
                          author,
                          &metadata.tags,
                          &independent_tags,
                          &metadata.data,
                          &descriptor.data,
                          &metadata.styles,
                          &descriptor.styles,
                          &metadata.scripts,
                          &descriptor.scripts)?
                .into_iter()
                .filter(|l| bloguen::util::is_asset_link(l)) {
                if let Ok(link) = percent_decode(link.as_bytes()).decode_utf8() {
                    if !p.copy_asset(&opts.output_dir, descriptor.asset_dir_override.as_ref().map(|s| &s[..]), &link)? {
                        eprintln!("Couldn't find \"{}\" for \"{}\" post.", link, p.normalised_name());
                    }
                } else {
                    eprintln!("Invalid percent-encoded \"{}\" link.", link);
                }
            }

            if descriptor.index.is_some() {
                idx_sender.send((p.number.clone(), index_machine_json, center_buffer))
                    .map_err(|e| {
                        bloguen::Error::Io {
                            desc: format!("post {} JSON metadata ", p.number.1).into(),
                            op: "save",
                            more: e.to_string().into(),
                        }
                    })?;
            }

            Ok(())
        })?;

    if let Some(idx) = descriptor.index.as_ref() {
        let mut posts_data: Vec<_> = idx_receiver.into_iter().collect();
        posts_data.sort_unstable_by_key(|&((num, _), ..)| num);

        let index_script = [bloguen::ops::ScriptElement::from_literal(String::from_utf8(posts_data.iter_mut()
                                    .fold("const BLOGUEN_POSTS = [".as_bytes().to_vec(), |mut acc, ((_, ref ns), ref mut metadata, ..)| {
                    let mut md = vec![];
                    swap(&mut md, metadata);
                    acc.extend(md);

                    if ns != &posts[posts.len() - 1].number.1 {
                        acc.extend(",\n".as_bytes());
                    } else {
                        acc.extend("];".as_bytes());
                    }

                    acc
                })).map_err(|e| {
                bloguen::Error::Parse {
                    tp: "UTF-8 string",
                    wher: "index file post metadata".into(),
                    more: e.to_string().into(),
                }
            })?)];

        let mut index_file = File::create(opts.output_dir.1.join("index.html")).map_err(|e| {
                bloguen::Error::Io {
                    desc: "output index file".into(),
                    op: "create",
                    more: e.to_string().into(),
                }
            })?;
        let index_date = Utc::now();
        bloguen::ops::format_output(index_header.as_ref().unwrap(),
                                    &descriptor.name,
                                    &global_language,
                                    &[&descriptor.data, &idx.data],
                                    "index",
                                    0,
                                    "index",
                                    &global_author,
                                    &index_date,
                                    &[],
                                    &[&descriptor.styles, &idx.styles],
                                    &[&descriptor.scripts, &idx.scripts, &index_script],
                                    &mut index_file,
                                    "index")?;

        {
            let write_center = |&(_, _, ref center): &(_, _, Vec<u8>)| {
                index_file.write_all(&center)
                    .map_err(|e| {
                        bloguen::Error::Io {
                            desc: "output index file center".into(),
                            op: "write",
                            more: e.to_string().into(),
                        }
                    })
            };
            match idx.center_order {
                bloguen::ops::CenterOrder::Forward => Result::from_iter(posts_data.iter().map(write_center))?,
                bloguen::ops::CenterOrder::Backward => Result::from_iter(posts_data.iter().rev().map(write_center))?,
            }
        }

        bloguen::ops::format_output(index_footer.as_ref().unwrap(),
                                    &descriptor.name,
                                    &global_language,
                                    &[&descriptor.data, &idx.data],
                                    "index",
                                    0,
                                    "index",
                                    &global_author,
                                    &index_date,
                                    &[],
                                    &[&descriptor.styles, &idx.styles],
                                    &[&descriptor.scripts, &idx.scripts, &index_script],
                                    &mut index_file,
                                    "index")?;
    }

    Ok(())
}
