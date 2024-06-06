use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    myvar: String,
}

fn main() {
    let args = Cli::parse();
    println!(
        "the rust code compiled, built, and ran with myvar={}",
        args.myvar
    );
}
