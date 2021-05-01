use anyhow::{Context, Result};
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, ContentArrangement, Table};
use linapi::system::modules::{LoadedModule, ModuleFile};
use once_cell::sync::OnceCell;
use std::{
    env,
    path::{Path, PathBuf},
};
use structopt::{clap::AppSettings, StructOpt};

static TERM_WIDTH: OnceCell<u16> = OnceCell::new();

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
    table.set_table_width(*TERM_WIDTH.get_or_init(|| match table.get_table_width() {
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
    pager::Pager::with_default_pager("less").setup();
    let mut table = create_table()?;
    table.set_header(&["Module", "Size (Bytes)", "References", "Used By"]);

    for m in LoadedModule::get_loaded()? {
        table.add_row(&[
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

    println!("{}", table);
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
    pager::Pager::with_default_pager("less").setup();
    let mut table = create_table()?;
    let m = get_module(name, uname)?;
    let info = m.info();
    // FIXME: amdgpu dependencies are comma separated single string?
    // Bug in linapi? kernel oddity?
    // panic!("{:?}", &info.dependencies);

    table.set_header(&["File".into(), m.path().display().to_string()]);

    table.add_row(&["Authors", &info.authors.join("\n")]);
    table.add_row(&["License", &info.license]);
    table.add_row(&["Description", &info.description]);
    table.add_row(&["Version", &info.version]);
    table.add_row(&["Firmware", &info.firmware.join("\n")]);
    table.add_row(&["Alias", &info.alias.join("\n")]);
    table.add_row(&["Dependencies", &info.dependencies.join("\n")]);
    table.add_row(&[
        "Soft Dependencies".into(),
        info.soft_dependencies.join("\n"),
    ]);
    table.add_row(&["In Tree", &info.in_tree.to_string()]);
    table.add_row(&["Retpoline", &info.retpoline.to_string()]);
    table.add_row(&["Staging", &info.staging.to_string()]);
    table.add_row(&["Version Magic", &info.version_magic]);
    table.add_row(&["Source Checksum", &info.source_checksum]);

    let mut p_table = create_table()?;
    p_table.set_header(&["Name", "Description", "Type"]);
    p_table.set_table_width(
        table.get_table_width().unwrap()
            // Get width of first column, we're second.
            - table.get_column(0).unwrap().get_max_content_width()
            // 6 Is how many characters, including padding, the first column borders take.
            // Plus 1 for our own padding, for a total of 7.
            - 7,
    );

    let mut parameters = info.parameters.clone();
    parameters.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    for p in parameters {
        let desc = p
            .description
            .as_ref()
            .map(|s| s.replace("\t", "    "))
            .unwrap_or_else(|| "None".into());
        p_table.add_row(&[p.name.clone(), desc, p.type_.clone()]);
    }
    if info.parameters.is_empty() {
        table.add_row(&["Parameters", "None"]);
    } else {
        table.add_row(&["Parameters", &p_table.to_string()]);
    }
    table.add_row(&["Signature", &m.has_signature().to_string()]);

    println!("{}", table);
    Ok(())
}

#[quit::main]
fn main() -> Result<()> {
    let args: Args = Args::from_args();
    let _ = create_table(); // So thatTERM_WIDTH gets set.

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
