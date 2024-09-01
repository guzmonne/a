use clap::Parser;

mod anthropic;
mod args;
mod error;
mod openai;
mod prelude;

use crate::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();
    let api: Api = args.globals.api.parse()?;

    match api {
        Api::OpenAi => openai::run(args).await,
        Api::Anthropic => anthropic::run(args).await,
    }
}
