use regex::Regex;
use std::io::Read;

use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena, ComrakOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hours_re = Regex::new(r"(?P<hours>\d+)h")?;

    let date_re = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})")?;

    let tag_re = Regex::new(r"#(?P<tag>[A-Za-z0-9_-]+)")?;

    let ws_re = Regex::new(r"^(\s|:)+|(\s|:)+$")?;

    let mut text = String::new();
    std::io::stdin().read_to_string(&mut text)?;

    let arena = Arena::new();

    let root = parse_document(&arena, &text, &ComrakOptions::default());

    fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &mut F)
    where
        F: FnMut(&'a AstNode<'a>),
    {
        f(node);
        for c in node.children() {
            iter_nodes(c, f);
        }
    }

    let mut hours = 0;
    let mut date: Option<String> = None;
    let mut description: Option<String> = None;
    let mut tag: Option<String> = None;

    let process = |date: &mut Option<String>,
                   description: &mut Option<String>,
                   tag: &Option<String>,
                   hours: &mut i32| {
        if *hours > 0 {
            if let (Some(date_s), Some(description_s)) = (date.as_ref(), description.as_ref()) {
                #[derive(serde::Serialize)]
                struct Record<'s> {
                    date: &'s String,
                    month: &'s str,
                    hours: i32,
                    tag: &'s Option<String>,
                    description: &'s String,
                }
                //eprintln!("=> {date_s} {hours} {tag:?} {description_s}");
                println!(
                    "{}",
                    serde_json::to_string(&Record {
                        date: date_s,
                        month: &date_s[0..7],
                        hours: *hours,
                        tag,
                        description: description_s,
                    })
                    .unwrap()
                );
                description.take();
                *hours = 0;
            }
        }
    };

    iter_nodes(root, &mut |node| {
        if let &NodeValue::Item(_) = &node.data.borrow().value {
            process(&mut date, &mut description, &tag, &mut hours);
        }

        if let &NodeValue::Text(ref text) = &node.data.borrow().value {
            if let Some(captures) = hours_re.captures(text) {
                hours += captures["hours"].parse::<i32>().unwrap();
            }

            if let Some(captures) = date_re.captures(text) {
                date = Some(format!(
                    "{}-{}-{}",
                    &captures["year"], &captures["month"], &captures["day"]
                ));
            }

            if let Some(captures) = tag_re.captures(text) {
                tag = Some(captures["tag"].into());
            }

            description = Some(
                ws_re
                    .replace(
                        &tag_re.replace(&hours_re.replace(&date_re.replace(text, ""), ""), ""),
                        "",
                    )
                    .into(),
            );
        }
    });
    process(&mut date, &mut description, &tag, &mut hours);

    Ok(())
}
