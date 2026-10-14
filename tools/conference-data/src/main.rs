use clap::{Parser, Subcommand};
use conference_data::{Result, collect, export, itinerary, navigation};
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(
    version,
    about = "EuroRust facts and planning resources; unknowns remain explicit"
)]
struct Args {
    #[arg(long, default_value = ".", global = true)]
    root: PathBuf,
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    /// Fetch official sources, validate fully, then publish an immutable snapshot.
    Refresh {
        #[arg(long, default_value = ".cache/http")]
        cache: PathBuf,
        #[arg(long)]
        accept_removals: bool,
    },
    /// Check dataset construction, digests, exports, and immutable snapshot.
    Validate,
    /// Regenerate exports offline from a checked catalog.
    Export {
        #[arg(long)]
        catalog: Option<PathBuf>,
    },
    /// Generate a deterministic starting itinerary from a participant profile.
    Recommend {
        #[arg(long)]
        profile: PathBuf,
        #[arg(long)]
        output: PathBuf,
        /// Also render a source-linked readable itinerary.
        #[arg(long)]
        markdown: Option<PathBuf>,
    },
    /// Check an agent's or participant's itinerary against its source snapshot.
    CheckItinerary {
        #[arg(long)]
        itinerary: PathBuf,
    },
    /// Check public Markdown links, document classes, and private-path boundaries.
    CheckDocs,
}
fn run(args: Args) -> Result<()> {
    match args.command {
        Command::Refresh {
            cache,
            accept_removals,
        } => {
            let checked = collect::refresh(&args.root, &args.root.join(cache), accept_removals)?;
            if args.root.join("data/catalog.json").exists() {
                let old = collect::load(&args.root.join("data/catalog.json"))?;
                println!(
                    "{}",
                    serde_json::to_string_pretty(&export::diff(&old, &checked)?)?
                );
            }
            let manifest = export::publish(checked, &args.root)?;
            println!(
                "Published {}: {} entries, {} fetched detail pages",
                manifest.snapshot_id.as_str(),
                manifest.session_count,
                manifest.detail_count
            );
        }
        Command::Validate => {
            let verified = export::verify(&args.root)?;
            let manifest = verified.manifest();
            println!(
                "Validated {}: {} entries",
                manifest.snapshot_id.as_str(),
                manifest.session_count
            );
        }
        Command::Export { catalog } => {
            let checked =
                collect::load(&catalog.unwrap_or_else(|| args.root.join("data/catalog.json")))?;
            export::publish(checked, &args.root)?;
            println!("Exports regenerated from checked catalog");
        }
        Command::Recommend {
            profile,
            output,
            markdown,
        } => {
            let checked = export::verify(&args.root)?;
            let profile = serde_json::from_slice(&fs::read(profile)?)?;
            let plan = itinerary::recommend(&checked, profile)?;
            if let Some(path) = markdown {
                fs::write(path, itinerary::markdown(&checked, &plan)?)?;
            }
            fs::write(
                &output,
                format!("{}\n", serde_json::to_string_pretty(&plan)?),
            )?;
            println!(
                "Wrote {} selections to {}",
                plan.selections.len(),
                output.display()
            );
        }
        Command::CheckItinerary { itinerary: path } => {
            let checked = export::verify(&args.root)?;
            let plan = serde_json::from_slice(&fs::read(path)?)?;
            for warning in itinerary::check(&checked, &plan)? {
                println!("NOTE: {warning}");
            }
            println!("Schedule feasibility checked; no booking or attendance claim");
        }
        Command::CheckDocs => navigation::check(&args.root)?,
    }
    Ok(())
}
fn main() {
    if let Err(error) = run(Args::parse()) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
