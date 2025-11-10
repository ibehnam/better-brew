use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::Command;
use tokio::sync::Semaphore;

/// Maximum number of concurrent package operations to prevent CPU overload
const MAX_CONCURRENT_OPERATIONS: usize = 4;

/// Better Brew - Parallel Homebrew package manager
#[derive(Parser)]
#[command(name = "bbrew")]
#[command(about = "Parallel Homebrew package downloads and upgrades", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Update Homebrew and fetch latest package definitions
    Update,
    /// Upgrade outdated packages in parallel
    Upgrade,
    /// Install packages in parallel
    Install {
        /// List of packages to install
        packages: Vec<String>,
    },
    /// Reinstall packages in parallel
    Reinstall {
        /// Reinstall all installed packages
        #[arg(short, long)]
        all: bool,
        /// List of packages to reinstall (ignored if --all is specified)
        packages: Vec<String>,
    },
}

/// Represents outdated formulae from `brew outdated --json`
#[derive(Debug, Deserialize)]
struct OutdatedPackages {
    formulae: Vec<Package>,
    casks: Vec<Package>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
}

/// Check if Homebrew is installed and accessible
async fn check_homebrew() -> Result<()> {
    let output = Command::new("which")
        .arg("brew")
        .output()
        .await
        .context("Failed to execute 'which brew'")?;

    if !output.status.success() {
        return Err(anyhow!(
            "Homebrew is not installed or not in PATH. Please install Homebrew first:\n\
             https://brew.sh"
        ));
    }

    Ok(())
}

/// Execute a command and stream output to stdout/stderr
async fn run_command(cmd: &str, args: &[&str]) -> Result<()> {
    println!("Running: {} {}", cmd, args.join(" "));

    let status = Command::new(cmd)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await
        .context(format!("Failed to execute: {} {}", cmd, args.join(" ")))?;

    if !status.success() {
        return Err(anyhow!("Command failed: {} {}", cmd, args.join(" ")));
    }

    Ok(())
}

/// Get list of outdated packages from Homebrew
async fn get_outdated_packages() -> Result<Vec<String>> {
    println!("Checking for outdated packages...");

    let output = Command::new("brew")
        .args(["outdated", "--json"])
        .output()
        .await
        .context("Failed to execute 'brew outdated --json'")?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to get outdated packages: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let outdated: OutdatedPackages = serde_json::from_slice(&output.stdout)
        .context("Failed to parse JSON output from brew outdated")?;

    let mut packages = Vec::new();
    packages.extend(outdated.formulae.into_iter().map(|p| p.name));
    packages.extend(outdated.casks.into_iter().map(|p| p.name));

    Ok(packages)
}

/// Get list of installed packages from Homebrew (formulae only, not casks)
async fn get_installed_packages() -> Result<Vec<String>> {
    println!("Getting list of installed packages...");

    let output = Command::new("brew")
        .args(["list", "--formula", "-1"])
        .output()
        .await
        .context("Failed to execute 'brew list --formula -1'")?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to get installed packages: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let packages: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(packages)
}

/// Fetch a single package in the background
async fn fetch_package(package: &str, semaphore: Arc<Semaphore>, pb: ProgressBar) -> Result<()> {
    let _permit = semaphore.acquire().await.unwrap();

    pb.set_message(format!("Fetching {}", package));

    let output = Command::new("brew")
        .args(["fetch", package])
        .output()
        .await
        .context(format!("Failed to fetch package: {}", package))?;

    if output.status.success() {
        pb.println(format!("✓ Fetched: {}", package));
        pb.inc(1);
        Ok(())
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        pb.println(format!("✗ Failed to fetch: {}", package));
        pb.inc(1);
        Err(anyhow!("Failed to fetch {}: {}", package, error_msg))
    }
}

/// Install a batch of packages in a single brew command
async fn install_package_batch(
    packages: Vec<String>,
    semaphore: Arc<Semaphore>,
    pb: ProgressBar,
) -> Result<Vec<String>> {
    let _permit = semaphore.acquire().await.unwrap();

    let batch_str = packages.join(", ");
    pb.set_message(format!("Installing batch: {}", batch_str));

    let mut args = vec!["install"];
    args.extend(packages.iter().map(|s| s.as_str()));

    let output = Command::new("brew")
        .args(&args)
        .output()
        .await
        .context(format!("Failed to install batch: {}", batch_str))?;

    if output.status.success() {
        for package in &packages {
            pb.println(format!("✓ Installed: {}", package));
        }
        pb.inc(packages.len() as u64);
        Ok(vec![])
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        pb.println(format!("✗ Batch failed: {}", error_msg.trim()));
        pb.inc(packages.len() as u64);
        Ok(packages) // Return failed packages
    }
}

/// Reinstall a batch of packages in a single brew command
async fn reinstall_package_batch(
    packages: Vec<String>,
    semaphore: Arc<Semaphore>,
    pb: ProgressBar,
) -> Result<Vec<String>> {
    let _permit = semaphore.acquire().await.unwrap();

    let batch_str = packages.join(", ");
    pb.set_message(format!("Reinstalling batch: {}", batch_str));

    let mut args = vec!["reinstall"];
    args.extend(packages.iter().map(|s| s.as_str()));

    let output = Command::new("brew")
        .args(&args)
        .output()
        .await
        .context(format!("Failed to reinstall batch: {}", batch_str))?;

    if output.status.success() {
        for package in &packages {
            pb.println(format!("✓ Reinstalled: {}", package));
        }
        pb.inc(packages.len() as u64);
        Ok(vec![])
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        pb.println(format!("✗ Batch failed: {}", error_msg.trim()));
        pb.inc(packages.len() as u64);
        Ok(packages) // Return failed packages
    }
}

/// Parallel update command - updates Homebrew itself
async fn update() -> Result<()> {
    println!("=== Better Brew Update ===\n");

    check_homebrew().await?;

    // Run brew update
    run_command("brew", &["update"]).await?;

    println!("\n✓ Update complete!");
    Ok(())
}

/// Parallel upgrade command - fetches packages in parallel then upgrades
async fn upgrade() -> Result<()> {
    println!("=== Better Brew Upgrade ===\n");

    check_homebrew().await?;

    // Step 1: Update package definitions first
    println!("Updating package definitions...");
    run_command("brew", &["update"]).await?;
    println!();

    // Step 2: Get outdated packages
    let packages = get_outdated_packages().await?;

    if packages.is_empty() {
        println!("✓ All packages are up to date!");
        return Ok(());
    }

    println!(
        "Found {} outdated package(s): {}\n",
        packages.len(),
        packages.join(", ")
    );

    // Step 3: Fetch all packages in parallel (with concurrency limit)
    println!(
        "Fetching packages with {} concurrent operations...",
        MAX_CONCURRENT_OPERATIONS
    );

    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_OPERATIONS));
    let pb = ProgressBar::new(packages.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .unwrap()
            .progress_chars("#>-")
    );

    let fetch_tasks: Vec<_> = packages
        .iter()
        .map(|package| fetch_package(package, semaphore.clone(), pb.clone()))
        .collect();

    // Wait for all fetches to complete
    let results = futures::future::join_all(fetch_tasks).await;
    pb.finish_with_message("Fetching complete");

    // Check for any failures
    let mut failed = Vec::new();
    for (i, result) in results.iter().enumerate() {
        if let Err(e) = result {
            eprintln!("✗ Error: {}", e);
            failed.push(&packages[i]);
        }
    }

    if !failed.is_empty() {
        let failed_names: Vec<&str> = failed.iter().map(|s| s.as_str()).collect();
        eprintln!(
            "\nWarning: {} package(s) failed to fetch: {}",
            failed.len(),
            failed_names.join(", ")
        );
    }

    println!("\n=== Installing upgrades ===\n");

    // Step 4: Run brew upgrade (will use pre-fetched packages)
    run_command("brew", &["upgrade"]).await?;

    println!("\n✓ Upgrade complete!");
    Ok(())
}

/// Parallel install command - installs packages in parallel
async fn install(packages: Vec<String>) -> Result<()> {
    println!("=== Better Brew Install ===\n");

    check_homebrew().await?;

    if packages.is_empty() {
        return Err(anyhow!("No packages specified to install"));
    }

    println!(
        "Installing {} package(s)\n",
        packages.len()
    );

    // Batch packages to reduce lock contention while maintaining parallelism
    // Each batch runs `brew install pkg1 pkg2 pkg3...` which Homebrew handles efficiently
    const BATCH_SIZE: usize = 10;
    let batches: Vec<Vec<String>> = packages
        .chunks(BATCH_SIZE)
        .map(|chunk| chunk.to_vec())
        .collect();

    println!(
        "Installing in {} batch(es) with {} concurrent operations...",
        batches.len(),
        MAX_CONCURRENT_OPERATIONS
    );

    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_OPERATIONS));
    let pb = ProgressBar::new(packages.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .unwrap()
            .progress_chars("#>-")
    );

    let install_tasks: Vec<_> = batches
        .into_iter()
        .map(|batch| install_package_batch(batch, semaphore.clone(), pb.clone()))
        .collect();

    // Wait for all installs to complete
    let results = futures::future::join_all(install_tasks).await;
    pb.finish_with_message("Installation complete");

    // Collect failed packages
    let mut failed = Vec::new();
    for result in results {
        match result {
            Ok(failed_packages) => failed.extend(failed_packages),
            Err(e) => eprintln!("✗ Error: {}", e),
        }
    }

    let succeeded = packages.len() - failed.len();

    println!();
    if succeeded > 0 {
        println!("✓ Successfully installed {} package(s)", succeeded);
    }

    if !failed.is_empty() {
        eprintln!(
            "✗ {} package(s) failed to install: {}",
            failed.len(),
            failed.join(", ")
        );
        return Err(anyhow!("Some packages failed to install"));
    }

    println!("\n✓ Install complete!");
    Ok(())
}

/// Parallel reinstall command - reinstalls packages in parallel
async fn reinstall(all: bool, packages: Vec<String>) -> Result<()> {
    println!("=== Better Brew Reinstall ===\n");

    check_homebrew().await?;

    let packages_to_reinstall = if all {
        println!("Reinstalling ALL installed packages...\n");
        get_installed_packages().await?
    } else {
        if packages.is_empty() {
            return Err(anyhow!(
                "No packages specified to reinstall. Use --all to reinstall all packages"
            ));
        }
        packages
    };

    if packages_to_reinstall.is_empty() {
        println!("✓ No packages to reinstall!");
        return Ok(());
    }

    println!(
        "Reinstalling {} package(s)\n",
        packages_to_reinstall.len()
    );

    // Batch packages to reduce lock contention while maintaining parallelism
    // Each batch runs `brew reinstall pkg1 pkg2 pkg3...` which Homebrew handles efficiently
    const BATCH_SIZE: usize = 10;
    let batches: Vec<Vec<String>> = packages_to_reinstall
        .chunks(BATCH_SIZE)
        .map(|chunk| chunk.to_vec())
        .collect();

    println!(
        "Reinstalling in {} batch(es) with {} concurrent operations...",
        batches.len(),
        MAX_CONCURRENT_OPERATIONS
    );

    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_OPERATIONS));
    let pb = ProgressBar::new(packages_to_reinstall.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .unwrap()
            .progress_chars("#>-")
    );

    let reinstall_tasks: Vec<_> = batches
        .into_iter()
        .map(|batch| reinstall_package_batch(batch, semaphore.clone(), pb.clone()))
        .collect();

    // Wait for all reinstalls to complete
    let results = futures::future::join_all(reinstall_tasks).await;
    pb.finish_with_message("Reinstallation complete");

    // Collect failed packages
    let mut failed = Vec::new();
    for result in results {
        match result {
            Ok(failed_packages) => failed.extend(failed_packages),
            Err(e) => eprintln!("✗ Error: {}", e),
        }
    }

    let succeeded = packages_to_reinstall.len() - failed.len();

    println!();
    if succeeded > 0 {
        println!("✓ Successfully reinstalled {} package(s)", succeeded);
    }

    if !failed.is_empty() {
        eprintln!(
            "✗ {} package(s) failed to reinstall: {}",
            failed.len(),
            failed.join(", ")
        );
        return Err(anyhow!("Some packages failed to reinstall"));
    }

    println!("\n✓ Reinstall complete!");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Update => update().await,
        Commands::Upgrade => upgrade().await,
        Commands::Install { packages } => install(packages).await,
        Commands::Reinstall { all, packages } => reinstall(all, packages).await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_homebrew_check() {
        // This test will pass if Homebrew is installed
        // In a real environment, you'd mock the command execution
        let result = check_homebrew().await;
        // Just ensure it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_package_parsing() {
        let json = r#"{
            "formulae": [
                {"name": "wget"},
                {"name": "curl"}
            ],
            "casks": [
                {"name": "firefox"}
            ]
        }"#;

        let outdated: OutdatedPackages = serde_json::from_str(json).unwrap();
        assert_eq!(outdated.formulae.len(), 2);
        assert_eq!(outdated.casks.len(), 1);
        assert_eq!(outdated.formulae[0].name, "wget");
    }
}
