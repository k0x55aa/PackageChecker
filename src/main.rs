mod utils;
use clap::Parser;
#[derive(Parser, Debug)]
#[command(name = "dependencyCheck", version, about = "")]
struct ArgumentCli {
    #[arg(short, long, default_value = "")]
    filepath: String,
    #[arg(short, long, default_value = "")]
    package: String,
}
#[tokio::main]

async fn main(){
    let args = ArgumentCli::parse();
    if args.package == "npm"{
        utils::npm::NpmCheck(&args.filepath).await;
    }
}