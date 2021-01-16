use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(short = "h", long = "help")]
    pub help: bool,

    pub command: Option<String>,

    #[structopt(short="sc", long="search-content")]
    pub search_content: Option<String>,

    #[structopt(short="var", long="variant")]
    pub variant: Option<String>,

    #[structopt(short="t", long="type")]
    pub extension_type: Option<String>,
}