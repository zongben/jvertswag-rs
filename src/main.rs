use anyhow::Result;
use clap::Parser;
use cli::Args;
use converter::Converter;
use schema::SchemaParams;

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
    let schema = schema::create_schema(SchemaParams {
        root: args.root,
        path: args.path,
        method: args.method,
        body: args.body,
        header: args.header,
        query: args.query,
        param: args.param,
        res: args.res,
    })?;
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
