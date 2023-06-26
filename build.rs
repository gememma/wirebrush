use grass::{Options, OutputStyle};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let sass = grass::from_path(
        "scss/style.scss",
        &Options::default().style(OutputStyle::Compressed),
    )
    .expect("SCSS input successfully compiles");

    let mut out_path: PathBuf = env::var("OUT_DIR")
        .expect("OUT_DIR env var contains a value")
        .into();
    out_path.push("style.css");

    let mut stylesheet = File::create(out_path).expect("can create file at stylesheet out path");
    write!(stylesheet, "{}", sass).expect("able to write compiled stylesheet to created file");
}
