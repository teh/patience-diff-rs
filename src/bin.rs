use argh::FromArgs;
use chrono::offset::Utc;
use chrono::DateTime;
use patience_diff::{patience_diff, Hunk, Range};
use std::cmp::{max, min};
use std::path::{Path, PathBuf};

fn check_file(value: &str) -> Result<PathBuf, String> {
    let pa = Path::new(value);
    if pa.is_file() {
        Ok(pa.to_path_buf())
    } else {
        Err("does not appear to point to a file".to_string())
    }
}

#[derive(FromArgs)]
/// Patience diff implementation.
struct Args {
    #[argh(positional, from_str_fn(check_file))]
    a: PathBuf,
    #[argh(positional, from_str_fn(check_file))]
    b: PathBuf,

    /// size of surrounding context
    #[argh(option, default = "3", short = 'c')]
    context: usize,
}

fn main() {
    let args: Args = argh::from_env();
    // split at "\n"
    let data_a = std::fs::read(&args.a).expect("Could not read first file");
    let data_b = std::fs::read(&args.b).expect("Could not read Second file");
    let lines_a = data_a.split(|x| *x == 0x0au8).collect();
    let lines_b = data_b.split(|x| *x == 0x0au8).collect();
    let diff = patience_diff(&lines_a, &lines_b);

    let na = lines_a.len();
    let nb = lines_b.len();

    let modified_a: DateTime<Utc> = std::fs::metadata(&args.a)
        .expect("could not extract metadata for file a")
        .modified()
        .unwrap()
        .into();
    let modified_b: DateTime<Utc> = std::fs::metadata(&args.b)
        .expect("could not extract metadata for file b")
        .modified()
        .unwrap()
        .into();

    // First expand segments by number of context lines. We only really need to
    // collect context left or right because it will be the same, unless there
    // is overlap, in which case we're collapsing in the next step.

    // Now collapse overlapping sections
    let mut stack: Vec<Hunk> = Vec::new();

    // pretty print
    // https://en.wikipedia.org/wiki/Diff#Unified_format
    println!("--- {}\t{}", args.a.display(), modified_a.to_rfc3339()); // TODO timestamp
    println!("+++ {}\t{}", args.b.display(), modified_b.to_rfc3339());
    for hunk in diff.iter() {
        println!(
            "@@ -{},{} +{},{} @@",
            hunk.remove.start, // TODO
            hunk.remove.end - hunk.remove.start,
            hunk.insert.start,
            hunk.insert.end - hunk.insert.start
        );
        // TODO - context
        for i in hunk.remove.start..hunk.remove.end {
            println!("-{}", std::str::from_utf8(&lines_a[i]).unwrap());
        }
        for i in hunk.insert.start..hunk.insert.end {
            println!("+{}", std::str::from_utf8(&lines_b[i]).unwrap());
        }
    }
}
