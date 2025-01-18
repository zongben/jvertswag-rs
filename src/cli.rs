use clap::{value_parser, Parser};

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(long, short = 'R')]
    pub root: String,

    #[arg(long, short = 'P')]
    pub path: String,

    #[arg(long, short = 'X', value_parser = to_uppercase)]
    pub method: String,

    #[arg(long, short = 'd')]
    pub body: Option<String>,

    #[arg(long, short = 'H')]
    pub header: Vec<String>,

    #[arg(long, short = 'q')]
    pub query: Option<Vec<String>>,

    #[arg(long, short = 'p')]
    pub param: Option<Vec<String>>,

    #[arg(long, short = 'g', value_parser = value_parser!(u8).range(1..=10), default_value = "2")]
    pub gap: u8,

    #[arg(long, short = 'c', default_value = "")]
    pub comment: String,

    #[arg(long, short = 'r')]
    pub res: Option<String>,

    #[arg(long, short = 'o', default_value = "0")]
    pub offset: u8,
}

fn to_uppercase(s: &str) -> Result<String, String> {
    Ok(s.to_uppercase())
}
