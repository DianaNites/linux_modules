use std::{
    env,
    io::{stdout, Write},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, ContentArrangement, Table};
use linapi::system::modules::{LoadedModule, ModuleFile};
use once_cell::sync::OnceCell;
use structopt::{clap::AppSettings, StructOpt};

static TERM_WIDTH: OnceCell<u16> = OnceCell::new();

// TODO: Figure out how to update to 3.x
#[derive(Debug, StructOpt)]
enum Commands {
    /// List loaded kernel modules
    List {
        // Even though subcommands are supposed to count as arguments?
        // With `ArgRequiredElseHelp`, `nms list` just displays help,
        // so add a dummy default value
        #[structopt(hidden(true), default_value("0"))]
        _hidden_clap_oddity: u8,
    },

    /// Load a kernel module
    Insert {
        /// Path to file, or module name.
        ///
        /// If this is a full path, that file is loaded directly.
        ///
        /// If a module name, `lib/modules/(uname -r)` will be searched.
        module: PathBuf,

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
    Info {
        /// Uname to use instead of the current one.
        ///
        /// This may be required if you want information on modules regarding a
        /// kernel other than the one currently running.
        ///
        /// Alternatively the full path to the module may be specified.
        #[structopt(long, short)]
        uname: Option<String>,

        /// Name of the module, or a path to one.
        module: PathBuf,
    },
}

/// Manage Linux Kernel Modules
#[derive(Debug, StructOpt)]
#[structopt(global_settings(&[
        AppSettings::ColoredHelp,
        AppSettings::GlobalVersion,
        AppSettings::VersionlessSubcommands,
        AppSettings::SubcommandsNegateReqs,
        AppSettings::DisableHelpSubcommand,
        AppSettings::ArgRequiredElseHelp,
    ]
))]
struct Args {
    /// Module name to load. For linux kernel support.
    #[structopt(hidden(true), required_unless("subcommand"))]
    name: Option<PathBuf>,

    /// Does nothing. For linux kernel support.
    #[structopt(short, hidden(true), required_unless("subcommand"))]
    #[allow(dead_code)]
    quiet: bool,

    #[structopt(subcommand)]
    cmd: Option<Commands>,
}

fn create_table() -> Result<Table> {
    let cols = env::var("COLUMNS")
        .map(|s| s.parse())
        .unwrap_or(Ok(80))
        .context("COLUMNS was not a valid number")?;
    let mut table = Table::new();
    table.set_width(*TERM_WIDTH.get_or_init(|| match table.width() {
        Some(w) => w,
        None => cols,
    }));
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .apply_modifier(UTF8_ROUND_CORNERS);
    Ok(table)
}

fn get_module(name: &Path, uname: Option<&str>) -> Result<ModuleFile> {
    if name.is_absolute() {
        Ok(ModuleFile::from_path(name)?)
    } else {
        // This unwrap should be okay, `name` always valid utf-8?
        if let Some(uname) = uname {
            Ok(ModuleFile::from_name_with_uname(
                name.to_str().unwrap(),
                uname,
            )?)
        } else {
            Ok(ModuleFile::from_name(name.to_str().unwrap())?)
        }
    }
}

fn list_modules() -> Result<()> {
    let mut table = create_table()?;
    table.set_header(["Module", "Size (Bytes)", "References", "Used By"]);

    for m in LoadedModule::get_loaded()? {
        table.add_row([
            m.name(),
            &m.size().to_string(),
            &m.ref_count().unwrap().to_string(),
            &m.holders()
                .iter()
                .map(|m| m.name())
                .collect::<Vec<_>>()
                .join("\n"),
        ]);
    }
    pager::Pager::with_default_pager("less").setup();
    let _ = writeln!(stdout(), "{}", table);
    Ok(())
}

fn add_module(name: &Path, force: bool) -> Result<()> {
    let m = get_module(name, None)?;
    if force {
        unsafe { m.force_load("")? };
    } else {
        m.load("")?;
    }
    Ok(())
}

fn remove_module(name: &str, force: bool) -> Result<()> {
    for name in &[
        name.to_string(),
        name.replace('-', "_"),
        name.replace('_', "-"),
    ] {
        let m = match LoadedModule::from_name(name) {
            Ok(m) => m,
            Err(_) => continue,
        };
        if force {
            unsafe { m.force_unload()? };
        } else {
            m.unload()?;
        }
        break;
    }
    Ok(())
}

fn info_module(name: &Path, uname: Option<&str>) -> Result<()> {
    let mut table = create_table()?;
    let m = get_module(name, uname)?;
    pager::Pager::with_default_pager("less").setup();
    let info = m.info();
    // FIXME: amdgpu dependencies are comma separated single string?
    // Bug in linapi? kernel oddity?
    // panic!("{:?}", &info.dependencies);

    table.set_header(&["File".into(), m.path().display().to_string()]);

    table.add_rows([
        ["Authors", &info.authors.join("\n")],
        ["License", &info.license],
        ["Description", &info.description],
        ["Version", &info.version],
        ["Firmware", &info.firmware.join("\n")],
        ["Alias", &info.alias.join("\n")],
        ["Dependencies", &info.dependencies.join("\n")],
        ["Soft Dependencies", &info.soft_dependencies.join("\n")],
        ["In Tree", &info.in_tree.to_string()],
        ["Retpoline", &info.retpoline.to_string()],
        ["Staging", &info.staging.to_string()],
        ["Version Magic", &info.version_magic],
        ["Source Checksum", &info.source_checksum],
    ]);

    let mut p_table = create_table()?;
    p_table.set_header(["Name", "Description", "Type"]);

    // Max width of `tables` contents before we add params
    let max_width = table
        .column_max_content_widths()
        .iter()
        .copied()
        .max()
        .unwrap_or(0);

    p_table.set_width(table.width().unwrap() - max_width);

    let mut parameters = info.parameters.clone();
    parameters.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    for p in parameters {
        let desc = p
            .description
            .as_ref()
            .map(|s| s.replace('\t', "    "))
            .unwrap_or_else(|| "None".into());
        p_table.add_row(&[p.name.clone(), desc, p.type_.clone()]);
    }
    if info.parameters.is_empty() {
        table.add_row(["Parameters", "None"]);
    } else {
        table.add_row(["Parameters", &p_table.to_string()]);
    }
    table.add_row(["Signature", &m.has_signature().to_string()]);

    let _ = writeln!(stdout(), "{}", table);
    Ok(())
}

#[quit::main]
fn main() -> Result<()> {
    let args: Args = Args::from_args();
    let _ = create_table(); // So that TERM_WIDTH gets set.

    if args.cmd.is_some() {
        match args.cmd.unwrap() {
            Commands::List { .. } => {
                list_modules().with_context(|| "Couldn't list running modules")?
            }
            Commands::Insert { module, force } => add_module(&module, force)
                .with_context(|| format!("Couldn't add module {}", module.display()))?,
            Commands::Remove { name, force } => remove_module(&name, force)
                .with_context(|| format!("Couldn't remove module {}", name))?,
            Commands::Info { module, uname } => info_module(&module, uname.as_deref())
                .with_context(|| {
                    format!("Couldn't get information on module {}", module.display())
                })?,
        }
    } else {
        // This exists for support with the Linux kernel, which calls modprobe as so:
        // `modprobe -q <name>`

        // Can't be `None`, guaranteed by clap requirements.
        let _ = add_module(&args.name.unwrap(), false);
        quit::with_code(1);
    }

    Ok(())
}
