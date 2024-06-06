use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    myvar: Option<String>,

    #[arg(short, long)]
    secret: Option<String>,
}

static NONE: &str = "<none>";

fn main() {
    let args = Cli::parse();
    println!(
        "the rust code compiled, built, and ran with myvar={} and mysecret={}",
        args.myvar.unwrap_or(NONE.to_owned()),
        args.secret.unwrap_or(NONE.to_owned())
    );
}
