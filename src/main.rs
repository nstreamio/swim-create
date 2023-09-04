use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the project
    #[arg(short, long)]
    name: String,
    #[arg(short, long, default_value = "test")]
    plane: String,
    #[arg(short, long, default_value_t = 9001)]
    port: u16,
    #[arg(short, long, default_value = "4.0.1")]
    version: String,
}


fn main() {
    let args = Args::parse();

    println!("Hello {}!", args.name)
}
