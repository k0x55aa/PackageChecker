mod utils;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "PackageChecker", version="0.1", about = "It checks for package in npm public repo")]
struct ArgumentCli {
    /// path to the package.json file
    #[arg(short, long, default_value = "")]
    filepath: String,
    /// package name eg: npm
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