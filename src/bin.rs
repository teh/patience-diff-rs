use argh::FromArgs;
use patience_diff::patience_diff;
use std::path::{Path, PathBuf};

fn check_file(value: &str) -> Result<PathBuf, String> {
    let pa = Path::new(value);
    if pa.is_file() {
        Ok(pa.to_path_buf())
    } else {
        Err("Please provide path to a file".to_string())
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
    #[argh(option, default = "3", short = 'u')]
    unified: usize,
}

fn main() {
    let args: Args = argh::from_env();
    // split at "\n"
    let a_lines = std::fs::read(args.a).expect("Could not read first file");
    let b_lines = std::fs::read(args.b).expect("Could not read Second file");
    println!(
        "{:?}",
        patience_diff(
            a_lines.split(|x| *x == 0x0au8).collect(),
            b_lines.split(|x| *x == 0x0au8).collect()
        )
    );
}
