use anyhow::{Context, Error, Result};
use clap::{ArgAction, ArgMatches, FromArgMatches, Parser, Subcommand, ValueHint};
use serde::{Deserialize, Serialize};
use std::{env, fs};

pub fn run(matches: &ArgMatches) -> anyhow::Result<()> {
    let cmd = Commands::from_arg_matches(matches)?;
    match cmd {
        Commands::Set(config) => write(config)?,
    }
    Ok(())
}

pub fn write(new_config: Config) -> Result<()> {
    let existing_config = load(&new_config.global);
    let config = match existing_config {
        Ok(saved_config) => new_config.merge(&saved_config),
        Err(_) => new_config,
    };
    let serialized_config = serde_json::to_string(&config)?;

    fs::write(file(&config.global)?, serialized_config + "\n")?;
    Ok(())
}

pub fn load(global: &bool) -> Result<Config, Error> {
    let path = file(global)
        .with_context(|| format!("Unable to get sfdx configuration path for {:?}", &global))?;

    let contents = fs::read_to_string(path.as_path()).context(format!(
        "Unable to read sfdx configuration from {}",
        &path.to_str().unwrap()
    ))?;
    Ok(serde_json::from_str::<Config>(&contents)?)
}

pub fn file(global: &bool) -> Result<std::path::PathBuf> {
    let file = directory(global)?.join("sfdx-config.json");
    Ok(file)
}
pub fn directory(global: &bool) -> Result<std::path::PathBuf> {
    let path = match global {
        false => env::current_dir().context("hi")?,
        true => dirs::home_dir()
            .expect("Could not find home directory. Unable to find global configuration."),
    }
    .join(".sfdx");
    Ok(path)
}

/// Update runtime configuration values for the CLI.
#[derive(Parser, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none", alias = "defaultusername")]
    #[arg(long, value_hint = ValueHint::Other)]
    target_org: Option<String>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        alias = "defaultdevhubusername"
    )]
    #[arg(long, value_hint = ValueHint::Other)]
    target_dev_hub: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", alias = "apiVersion")]
    #[arg(long, action=ArgAction::Set)]
    org_api_version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", alias = "restDeploy")]
    #[arg(long, action=ArgAction::Set, value_parser = ["true", "false"])]
    org_metadata_rest_deploy: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", alias = "disableTelemetry")]
    #[arg(long, value_parser = ["true", "false"])]
    disable_telemetry: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", alias = "instanceUrl")]
    #[arg(long, value_hint = ValueHint::Url)]
    org_instance_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", alias = "maxQueryLimit")]
    #[arg(long, value_hint = ValueHint::Other)]
    org_max_query_limit: Option<String>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        alias = "customOrgMetadataTemplates"
    )]
    #[arg(long)]
    org_custom_metadata_templates: Option<String>,

    #[serde(skip)]
    #[arg(short, long)]
    global: bool,
}

impl Config {
    /// Merges two configurations together.
    /// Prefers values from self if present
    #[must_use]
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            target_org: self.target_org.clone().or_else(|| other.target_org.clone()),
            target_dev_hub: self
                .target_dev_hub
                .clone()
                .or_else(|| other.target_dev_hub.clone()),
            org_api_version: self
                .org_api_version
                .clone()
                .or_else(|| other.org_api_version.clone()),
            org_metadata_rest_deploy: self
                .org_metadata_rest_deploy
                .clone()
                .or_else(|| other.org_metadata_rest_deploy.clone()),
            disable_telemetry: self
                .disable_telemetry
                .clone()
                .or_else(|| other.disable_telemetry.clone()),
            org_instance_url: self
                .org_instance_url
                .clone()
                .or_else(|| other.org_instance_url.clone()),
            org_max_query_limit: self
                .org_max_query_limit
                .clone()
                .or_else(|| other.org_max_query_limit.clone()),
            org_custom_metadata_templates: self
                .org_custom_metadata_templates
                .clone()
                .or_else(|| other.org_custom_metadata_templates.clone()),
            global: self.global,
        }
    }
}

#[derive(Parser)]
struct ListArgs {}

#[derive(Subcommand)]
pub enum Commands {
    // List(ListArgs),
    Set(Config),
    // Unset(Config),
}
#[derive(clap::ValueEnum, Clone, Debug, Copy)]
pub enum Location {
    Global,
    Project,
}
