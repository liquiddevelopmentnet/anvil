use clap::Parser;

#[derive(Parser)]
#[command(
    name = "anvil",
    author = "Finn Behrend",
    version = env!("CARGO_PKG_VERSION"),
    about = "CLI for interacting with anvil servers, anvil is a high-end, high performance deployment managing service and CI/CD.",
    long_about = None
)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand)]
enum Action {
}

fn main() {
    let args = Args::parse();
    match args.action {
    }
}
