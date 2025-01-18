use anyhow::Result;
use clap::Parser;
use cli::Args;
use converter::Converter;

mod cli;
mod converter;
mod http;
mod schema;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();
    let schema = schema::from_args(&args)?;
    let mut converter = Converter::new(args.gap, args.offset, &args.comment);
    let result = converter.convert_schema(&schema);
    match result {
        Ok(swag_schema) => {
            println!("{}", swag_schema);
            Ok(())
        }
        Err(e) => return Err(e),
    }
}
