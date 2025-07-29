use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Just a testing input
    name: String,

    /// Send fuck message
    #[arg(short, long, default_value_t=false)]
    send_fuck: bool
}

fn main() {
    let args = Cli::parse();
    println!("name: {:?}", args.name);
    if args.send_fuck {
        println!("Fuck you!")
    }
}
