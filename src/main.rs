use anyhow::Result;
use std::path::{Path, PathBuf};
use structopt::{clap::AppSettings, StructOpt};

#[derive(Debug, StructOpt)]
enum Commands {
    /// List loaded kernel modules
    List {},

    /// Load a kernel module
    Insert {
        /// Path to file, or module name.
        ///
        /// If this is a full path, that file is loaded directly.
        ///
        /// If a file name, `lib/modules/(uname -r)` will be searched.
        file: PathBuf,

        /// Force load the module.
        ///
        /// This ignores important kernel compatibility checks.
        ///
        /// USE AT YOUR OWN RISK
        #[structopt(long, short)]
        force: bool,
    },

    /// Remove/unload a kernel module
    Remove {
        /// Name of the module to unload
        name: String,

        /// Force unload the module.
        ///
        /// This is extremely dangerous and can cause serious system
        /// instability. Not all modules are designed to be unloaded, and this
        /// can cause a module currently being used to be unloaded.
        ///
        /// USE AT YOUR OWN RISK
        #[structopt(long, short)]
        force: bool,
    },

    /// Get information on a kernel module
    Info {},
}

/// Manage Linux Kernel Modules
#[derive(Debug, StructOpt)]
#[structopt(global_settings(&[
        AppSettings::ColoredHelp,
        AppSettings::GlobalVersion,
        AppSettings::DisableVersion,
    ]
))]
struct Args {
    #[structopt(subcommand)]
    cmd: Commands,
}

fn list_modules() {
    let mut table = Table::new();
    table.add_row(row!["Module", "Size (Bytes)", "References", "Used By"]);
    for m in get_system_modules() {
        let mut holders = String::new();
        for h in m.holders() {
            holders.push_str(&h.name());
            holders.push('\n');
        }
        table.add_row(Row::new(vec![
            Cell::new(&m.name()),
            Cell::new(&m.size().to_string()),
            Cell::new(&m.holders().len().to_string()),
            Cell::new(&holders),
        ]));
    }
    table.printstd();
}

fn add_module(name: &Path, force: bool) {
    todo!()
}

fn remove_module(name: &str, force: bool) {
    // TODO: Add API to `linapi` for unloading modules
    todo!()
}

fn main() -> Result<()> {
    let args = Args::from_args();
    //
    match args.cmd {
        Commands::List { .. } => list_modules(),
        Commands::Insert { file, force } => add_module(&file, force),
        Commands::Remove { name, force } => remove_module(&name, force),
        _ => todo!(),
    }
    //
    Ok(())
}
