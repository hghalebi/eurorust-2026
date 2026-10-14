use agent_contract_lab::{input::RawRequest, retry::*};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "Checked input and local retry-budget experiments")]
struct Args {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    Input {
        #[arg(long)]
        tool: String,
        #[arg(long)]
        timeout_ms: u64,
    },
    Retry {
        #[arg(long)]
        budget: u32,
        #[arg(long)]
        failures: u32,
    },
}
fn main() {
    match Args::parse().command {
        Command::Input { tool, timeout_ms } => match (RawRequest { tool, timeout_ms }).validate() {
            Ok(request) => println!("{request}"),
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(1);
            }
        },
        Command::Retry { budget, failures } => {
            let limit = BudgetLimit::try_from(budget).unwrap_or_else(|error| {
                eprintln!("{error}");
                std::process::exit(1)
            });
            let mut work = SimulatedWork::new(RemainingFailures::from(failures));
            match run(&mut work, AttemptBudget::new(limit)) {
                Outcome::Completed(receipt) => println!("{receipt}"),
                Outcome::Exhausted(used) => {
                    eprintln!("exhausted after {used} attempts");
                    std::process::exit(2);
                }
            }
        }
    }
}
