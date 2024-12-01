use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub day: i32,

    #[clap(long, short, action)]
    pub second: bool,

    #[clap(long, short, action)]
    pub test: bool,
}