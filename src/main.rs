mod config;
mod update;

use anyhow::{Context, Result};
use clap::Parser;
use config::Config;
use std::process::Command;

#[derive(Parser)]
#[command(name = "dusk-warden", about = "Sync Bitwarden secrets to .env files")]
enum Cli {
    /// Download secrets from Bitwarden and write them to configured .env files
    Sync,
    /// Upload local .env files to Bitwarden secrets
    Push,
    /// Update dusk-warden to the latest release
    Update,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli {
        Cli::Sync => sync()?,
        Cli::Push => push()?,
        Cli::Update => update::update()?,
    }

    Ok(())
}

fn sync() -> Result<()> {
    let config = Config::load()?;
    let token = config::resolve_bws_token()?;

    for secret in &config.secrets {
        let value = fetch_secret(&secret.id, &token)
            .with_context(|| format!("Failed to fetch secret for {}", secret.path))?;

        std::fs::write(&secret.path, value)
            .with_context(|| format!("Failed to write {}", secret.path))?;

        println!("Wrote {}", secret.path);
    }

    Ok(())
}

fn push() -> Result<()> {
    let config = Config::load()?;
    let token = config::resolve_bws_token()?;

    for secret in &config.secrets {
        let value = std::fs::read_to_string(&secret.path)
            .with_context(|| format!("Failed to read {}", secret.path))?;

        update_secret(&secret.id, &value, &token)
            .with_context(|| format!("Failed to push secret for {}", secret.path))?;

        println!("Pushed {}", secret.path);
    }

    Ok(())
}

fn update_secret(secret_id: &str, value: &str, token: &str) -> Result<()> {
    let output = Command::new("bws")
        .args(["secret", "edit", "--value", value, secret_id])
        .env("BWS_ACCESS_TOKEN", token)
        .output()
        .context("Failed to run bws CLI. Is it installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(check_bws_error(&stderr, secret_id));
    }

    Ok(())
}

fn check_bws_error(stderr: &str, secret_id: &str) -> anyhow::Error {
    if stderr.contains("404") || stderr.contains("Resource not found") {
        anyhow::anyhow!(
            "Secret {secret_id} not found or access denied. \
             Check that your service account token has write permissions."
        )
    } else {
        anyhow::anyhow!("bws failed: {stderr}")
    }
}

fn fetch_secret(secret_id: &str, token: &str) -> Result<String> {
    let output = Command::new("bws")
        .args(["secret", "get", secret_id])
        .env("BWS_ACCESS_TOKEN", token)
        .output()
        .context("Failed to run bws CLI. Is it installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("bws failed: {stderr}");
    }

    let json: serde_json::Value = serde_json::from_slice(&output.stdout)
        .context("Failed to parse bws output as JSON")?;

    json["value"]
        .as_str()
        .map(|s| s.to_string())
        .context("Secret value not found in bws output")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_bws_error_returns_permission_message_on_404() {
        let stderr = "Error: Received error message from server: [404 Not Found] {\"message\":\"Resource not found.\"}";
        let err = check_bws_error(stderr, "abc-123");
        let msg = err.to_string();
        assert!(msg.contains("not found or access denied"), "got: {msg}");
        assert!(msg.contains("write permissions"), "got: {msg}");
        assert!(msg.contains("abc-123"), "got: {msg}");
    }

    #[test]
    fn check_bws_error_returns_raw_stderr_for_other_errors() {
        let stderr = "Error: something else went wrong";
        let err = check_bws_error(stderr, "abc-123");
        let msg = err.to_string();
        assert!(msg.contains("bws failed:"), "got: {msg}");
        assert!(msg.contains("something else went wrong"), "got: {msg}");
    }
}
