use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    pub command: String,
    #[structopt(short="t", long="type")]
    pub extension_type: Option<String>
}