use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    run: String
}