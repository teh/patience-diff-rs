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

    // Empirically no diff results in no output at all in other diff tools
    if diff.len() == 0 {
        return;
    }

    println!("--- {}\t{}", args.a.display(), modified_a.to_rfc3339());
    println!("+++ {}\t{}", args.b.display(), modified_b.to_rfc3339());

    // Collapse overlapping sections. We only need to do this for one side
    // because overlap will be by construction the same on both sides.
    let mut stack: Vec<(usize, usize)> = vec![(
        max(0, diff[0].remove.start - args.context),
        min(na, diff[0].remove.end + args.context),
    )];
    for Hunk { remove, .. } in diff.iter().skip(1) {
        let mut top = stack.last_mut().unwrap();
        if top.1 > remove.start - args.context {
            (*top).1 = min(na, max(top.1, remove.end + args.context));
        } else {
            let new_top = (
                max(0, remove.start - args.context),
                min(remove.end + args.context, na),
            );
            stack.push(new_top);
        }
    }

    // Map overlapping sections back to the actual hunks. We know by
    // construction that they will line up so we can just left-merge.
    let mut i = 0;
    let mut j = 0;
    while i < stack.len() {
        let (start, end) = stack[i];
        let Hunk { remove, insert } = diff[j];
        println!(
            "@@ -{},{} +{},{} @@",
            start,
            end - start,
            insert.start - args.context,
            insert.end - insert.start
        );
        let mut inter_start = start;
        while j < diff.len() && diff[j].remove.end <= end {
            let Hunk { remove, insert } = diff[j];
            // only print as many lines as needed
            for pp in inter_start..remove.start {
                println!(" {}", std::str::from_utf8(&lines_a[pp]).unwrap());
            }
            for pp in remove.start..remove.end {
                println!("-{}", std::str::from_utf8(&lines_a[pp]).unwrap());
            }
            for pp in insert.start..insert.end {
                println!("+{}", std::str::from_utf8(&lines_b[pp]).unwrap());
            }
            inter_start = remove.end;
            // // only print as many lines as needed
            // for pp in remove.end..remove.end + args.context {
            //     println!(" {}", std::str::from_utf8(&lines_a[pp]).unwrap());
            // }
            j += 1;
        }
        for pp in inter_start..end {
            println!(" {}", std::str::from_utf8(&lines_a[pp]).unwrap());
        }
        i += 1
    }

    // for h in stack.iter() {
    //     println!("S {:?}", h);
    // }

    //     println!(
    //         "@@ -{},{} +{},{} @@",
    //         remove.start, // TODO
    //         remove.end - remove.start,
    //         insert.start,
    //         insert.end - insert.start
    //     );
    //     for i in remove.start..remove.end {
    //         println!("-{}", std::str::from_utf8(&lines_a[i]).unwrap());
    //     }
    //     for i in insert.start..insert.end {
    //         println!("+{}", std::str::from_utf8(&lines_b[i]).unwrap());
    //     }
    // }
    // pretty print
    // https://en.wikipedia.org/wiki/Diff#Unified_format
}
