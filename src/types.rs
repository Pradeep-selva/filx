use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    pub command: String
}