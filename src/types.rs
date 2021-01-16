use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
    pub command: String,
    #[structopt(short="var", long="variant")]
    pub variant: Option<String>,
    #[structopt(short="t", long="type")]
    pub extension_type: Option<String>
}