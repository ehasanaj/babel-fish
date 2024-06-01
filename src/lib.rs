use args::Args;
use clap::Parser;
use std::error::Error;

mod args;
mod request;

pub type EmptyResult = Result<(), Box<dyn Error>>;

pub async fn run() -> EmptyResult {
    let args = Args::parse();

    let translated_text = request::execute(args.destination(), args.text()).await?;
    println!("{translated_text}");

    Ok(())
}
