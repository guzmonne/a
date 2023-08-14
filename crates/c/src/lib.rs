use clap::{Parser, Subcommand, ValueEnum};

use anthropic::complete::Model as AnthropicModel;
use serde::{Deserialize, Serialize};

pub mod commands;
pub mod session;
pub mod utils;

#[derive(Debug, Parser)]
#[command(name = "v2")]
#[command(about = "Interact with OpenAI's ChatGPT through the terminal")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Anthropic Chat AI API
    #[clap(alias = "a")]
    Anthropic(commands::anthropic::CommandOptions),
    /// OpenAi Chat AI API
    #[clap(name = "openai", alias = "o")]
    OpenAi(commands::openai::CommandOptions),
}

#[derive(ValueEnum, Debug, Clone, Copy)]
#[clap(rename_all = "kebab-case")]
pub enum ClaudeModelOption {
    ClaudeV1,
    ClaudeV1_100k,
    ClaudeInstantV1,
    ClaudeInstantV1_100k,
}

impl From<ClaudeModelOption> for AnthropicModel {
    fn from(other: ClaudeModelOption) -> AnthropicModel {
        match other {
            ClaudeModelOption::ClaudeV1 => AnthropicModel::ClaudeV1,
            ClaudeModelOption::ClaudeV1_100k => AnthropicModel::ClaudeV1_100k,
            ClaudeModelOption::ClaudeInstantV1 => AnthropicModel::ClaudeInstantV1,
            ClaudeModelOption::ClaudeInstantV1_100k => AnthropicModel::ClaudeInstantV1_100k,
        }
    }
}

impl From<AnthropicModel> for ClaudeModelOption {
    fn from(other: AnthropicModel) -> ClaudeModelOption {
        match other {
            AnthropicModel::ClaudeV1 => ClaudeModelOption::ClaudeV1,
            AnthropicModel::ClaudeV1_100k => ClaudeModelOption::ClaudeV1_100k,
            AnthropicModel::ClaudeInstantV1 => ClaudeModelOption::ClaudeInstantV1,
            AnthropicModel::ClaudeInstantV1_100k => ClaudeModelOption::ClaudeInstantV1_100k,
        }
    }
}

#[derive(Default, ValueEnum, Debug, Clone, Serialize, Deserialize)]
#[clap(rename_all = "kebab-case")]
pub enum Output {
    #[default]
    /// Plain text
    Raw,
    /// JSON
    Json,
    /// YAML
    Yaml,
}
