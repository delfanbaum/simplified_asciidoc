use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "sdoc",
    version,
    about,
    long_about = "A rust coverter from simplified asciidoc to html."
)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args)
}
