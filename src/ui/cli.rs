use crate::{Experiment, Result, TrackerError};
use clap::{Parser, Subcommand};
use console::{style, Term};
use dialoguer::{Input, Select};
use uuid::Uuid;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Create a new experiment")]
    CreateExperiment {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        description: Option<String>,
    },

    #[command(about = "List all experiments")]
    ListExperiments,

    #[command(about = "Start a new run")]
    StartRun {
        #[arg(short, long)]
        experiment_id: Uuid,
    },

    #[command(about = "Show run details")]
    ShowRun {
        #[arg(short, long)]
        run_id: Uuid,
    },

    #[command(about = "List artifacts for a run")]
    ListArtifacts {
        #[arg(short, long)]
        run_id: Uuid,
    },
}

pub struct CliApp {
    term: Term,
}

impl From<std::io::Error> for TrackerError {
    fn from(err: std::io::Error) -> Self {
        TrackerError::Storage(err.to_string())
    }
}

impl Default for CliApp {
    fn default() -> Self {
        Self::new()
    }
}

impl CliApp {
    pub fn new() -> Self {
        Self {
            term: Term::stdout(),
        }
    }

    pub async fn run(&self) -> Result<()> {
        let cli = Cli::parse();

        match cli.command {
            Commands::CreateExperiment { name, description } => {
                self.create_experiment(name, description).await
            }
            Commands::ListExperiments => self.list_experiments().await,
            Commands::StartRun { experiment_id } => self.start_run(experiment_id).await,
            Commands::ShowRun { run_id } => self.show_run(run_id).await,
            Commands::ListArtifacts { run_id } => self.list_artifacts(run_id).await,
        }
    }

    async fn create_experiment(&self, name: String, description: Option<String>) -> Result<()> {
        let experiment = Experiment::new(name).with_description(description.unwrap_or_default());

        self.term.write_line(&format!(
            "{} Created experiment {} (ID: {})",
            style("✓").green(),
            style(&experiment.name).cyan(),
            experiment.id
        ))?;

        Ok(())
    }

    async fn list_experiments(&self) -> Result<()> {
        self.term.write_line("Experiments:")?;
        self.term.write_line("------------")?;
        Ok(())
    }

    async fn start_run(&self, _experiment_id: Uuid) -> Result<()> {
        let input = Input::<String>::new();
        loop {
            let add_param = Select::new()
                .with_prompt("Add parameter?")
                .items(&["Yes", "No"])
                .default(0)
                .interact()
                .map_err(|e| TrackerError::InvalidOperation(e.to_string()))?;

            if add_param == 1 {
                break;
            }

            let _key = input
                .clone()
                .with_prompt("Parameter name")
                .interact_text()
                .map_err(|e| TrackerError::InvalidOperation(e.to_string()))?;

            let _value = input
                .clone()
                .with_prompt("Parameter value")
                .interact_text()
                .map_err(|e| TrackerError::InvalidOperation(e.to_string()))?;
        }

        Ok(())
    }

    async fn show_run(&self, run_id: Uuid) -> Result<()> {
        self.term
            .write_line(&format!("Run Details (ID: {})", run_id))?;
        self.term.write_line("------------")?;
        Ok(())
    }

    async fn list_artifacts(&self, run_id: Uuid) -> Result<()> {
        self.term
            .write_line(&format!("Artifacts for Run {}", run_id))?;
        self.term.write_line("------------")?;
        Ok(())
    }
}
