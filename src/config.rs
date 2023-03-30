use anyhow::{Context, Error, Result};
use clap::{ArgAction, ArgMatches, FromArgMatches, Parser, Subcommand, ValueHint};
use serde::{Deserialize, Serialize};
use std::{env, fs};

pub fn run(matches: &ArgMatches) -> anyhow::Result<()> {
    let cmd = ConfigCommand::from_arg_matches(matches)?;
    match cmd {
        ConfigCommand::Set(config) => write_config(config)?,
    }
    Ok(())
}

pub fn write_config(new_config: Config) -> Result<()> {
    let existing_config = get_saved_config(&new_config.global);
    let config = match existing_config {
        Ok(saved_config) => new_config.merge(&saved_config),
        Err(_) => new_config,
    };
    let serialized_config = serde_json::to_string(&config)?;

    fs::write(
        get_config_filepath(&config.global)?,
        serialized_config + "\n",
    )?;
    Ok(())
}

pub fn get_saved_config(global: &bool) -> Result<Config, Error> {
    let config_path = get_config_filepath(global)
        .with_context(|| format!("Unable to get sfdx configuration path for {:?}", &global))?;

    let contents = fs::read_to_string(config_path.as_path()).context(
        format!(
            "Unable to read sfdx configuration from {}",
            &config_path.to_str().unwrap()
        )
        .to_owned(),
    )?;
    return Ok(serde_json::from_str::<Config>(&contents)?);
}

pub fn get_config_filepath(global: &bool) -> Result<std::path::PathBuf> {
    let config_file = get_config_dir(global)?.join("sfdx-config.json");
    Ok(config_file)
}
pub fn get_config_dir(global: &bool) -> Result<std::path::PathBuf> {
    let config_path = match global {
        false => env::current_dir().context("hi")?,
        true => dirs::home_dir()
            .expect("Could not find home directory. Unable to find global configuration."),
    }
    .join(".sfdx");
    Ok(config_path)
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
    pub fn merge(self: &Self, other: &Config) -> Self {
        Self {
            target_org: self.target_org.clone().or(other.target_org.clone()),
            target_dev_hub: self.target_dev_hub.clone().or(other.target_dev_hub.clone()),
            org_api_version: self
                .org_api_version
                .clone()
                .or(other.org_api_version.clone()),
            org_metadata_rest_deploy: self
                .org_metadata_rest_deploy
                .clone()
                .or(other.org_metadata_rest_deploy.clone()),
            disable_telemetry: self
                .disable_telemetry
                .clone()
                .or(other.disable_telemetry.clone()),
            org_instance_url: self
                .org_instance_url
                .clone()
                .or(other.org_instance_url.clone()),
            org_max_query_limit: self
                .org_max_query_limit
                .clone()
                .or(other.org_max_query_limit.clone()),
            org_custom_metadata_templates: self
                .org_custom_metadata_templates
                .clone()
                .or(other.org_custom_metadata_templates.clone()),
            global: self.global,
        }
    }
}

#[derive(Parser)]
struct ListArgs {}

#[derive(Subcommand)]
pub enum ConfigCommand {
    // List(ListArgs),
    Set(Config),
    // Unset(Config),
}
#[derive(clap::ValueEnum, Clone, Debug, Copy)]
pub enum ConfigLocation {
    Global,
    Project,
}
