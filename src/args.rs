use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "LKBaka", version, about = "a package manager for ant", long_about = None)]
pub(crate) struct Args {
    #[arg(short, long, default_value_t = false)]
    pub(crate) init: bool,

    #[arg(long)]
    pub(crate) add: Option<String>,
}
