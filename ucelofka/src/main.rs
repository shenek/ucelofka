pub mod actions;
pub mod storage;
pub mod translations;
pub mod web;

use anyhow::{anyhow, Result};
use clap::{
    crate_authors, crate_description, crate_name, crate_version, value_parser, Arg, ArgAction,
    ArgMatches, Command,
};
use clap_complete::{
    generate,
    shells::{Bash, Elvish, Fish, PowerShell, Zsh},
    Generator,
};
use fluent::fluent_args;
use std::io;
use std::path::{Path, PathBuf};
use ucelofka_data as data;

use crate::{
    actions::{account, customer, entry, identity, ids, invoice, project, template},
    translations::{get_message, texts},
};

pub fn check_data_dir(path: &PathBuf) -> Result<PathBuf> {
    let root_dir: &Path = path.as_path();
    let path_str = path.to_string_lossy().to_string();
    if let Ok(path) = root_dir.canonicalize() {
        if !path.is_dir() {
            return Err(anyhow!(get_message(
                "is-not-a-directory",
                Some(fluent_args!["path" => path_str])
            )));
        }
        for &subdir in &[
            "accounts",
            "customers",
            "entries",
            "identities",
            "invoices",
            "templates",
            "output",
        ] {
            let subdir_path = path.join(Path::new(subdir));
            if !subdir_path.is_dir() {
                return Err(anyhow!(get_message(
                    "data-directory-is-missing-subdir",
                    Some(fluent_args![
                        "dir_path" => root_dir.to_str().unwrap_or("?"),
                        "subdir_path" => subdir
                    ]),
                )));
            }
        }
        Ok(root_dir.to_owned())
    } else {
        Err(anyhow!(get_message(
            "path-not-exits",
            Some(fluent_args!["path" => path_str])
        )))
    }
}

fn prepare_data_dir() -> Arg {
    Arg::new("data_dir")
        .short('P')
        .long("path")
        .value_name("DATA_DIR")
        .num_args(1)
        .required(false)
        .value_parser(value_parser!(PathBuf))
        .help(&texts::DATA_DIRECTORY_PATH.to_string())
        .default_value(".")
}

fn prepare_get_subcommand(help: &'static str) -> Command {
    Command::new("get")
        .arg(
            Arg::new("id")
                .short('I')
                .long("id")
                .num_args(1)
                .required(true),
        )
        .about(help)
}

fn prepare_invoice_subcommand() -> Command {
    Command::new("invoice")
        .arg(prepare_data_dir())
        .about("Invoice management")
        .subcommand(
            Command::new("create")
                .about("Creates a new invoice")
                .arg(
                    Arg::new("customer")
                        .help("Customer id")
                        .long("customer")
                        .short('C')
                        .num_args(1)
                        .required(true),
                )
                .arg(
                    Arg::new("identity")
                        .help("Identity id")
                        .short('I')
                        .long("identity")
                        .num_args(1)
                        .required(true),
                )
                .arg(
                    Arg::new("account")
                        .help("Account id")
                        .long("account")
                        .short('A')
                        .num_args(1)
                        .required(true),
                )
                .arg(
                    Arg::new("entry")
                        .help("Entry id")
                        .short('E')
                        .long("entry")
                        .num_args(1)
                        .required(true)
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("git")
                        .env("UCELOFKA_GIT")
                        .help("Add newly created invoice to git")
                        .short('G')
                        .long("git")
                        .num_args(0)
                        .required(false),
                )
                .arg(
                    Arg::new("due")
                        .help("Due time (in days)")
                        .short('D')
                        .long("due")
                        .num_args(1)
                        .required(false)
                        .value_parser(value_parser!(usize)),
                ),
        )
        .subcommand(Command::new("list").about("Lists invoices"))
        .subcommand(Command::new("ids").about("List invoice ids"))
        .subcommand(
            Command::new("render")
                .about("Renders invoice")
                .arg(
                    Arg::new("template")
                        .help("Template id")
                        .short('T')
                        .long("template")
                        .num_args(1)
                        .required(true),
                )
                .arg(
                    Arg::new("invoice")
                        .help("Invoice id")
                        .short('I')
                        .long("invoice")
                        .num_args(1)
                        .required(true),
                )
                .arg(
                    Arg::new("git")
                        .env("UCELOFKA_GIT")
                        .help("Add newly created file to git")
                        .short('G')
                        .long("git")
                        .num_args(0)
                        .required(false),
                ),
        )
        .subcommand(prepare_get_subcommand("Get invoice"))
}

fn prepare_project_subcommand() -> Command {
    Command::new("project")
        .about("Manages data project")
        .subcommand(
            Command::new("make")
                .about("Creates new data dir")
                .arg(
                    Arg::new("target")
                        .help("Where is should be placed")
                        .short('T')
                        .long("target")
                        .value_parser(value_parser!(PathBuf))
                        .num_args(1)
                        .required(true),
                )
                .arg(
                    Arg::new("git")
                        .env("UCELOFKA_GIT")
                        .help("Initialize with a git repository")
                        .short('G')
                        .long("git")
                        .num_args(0)
                        .required(false),
                ),
        )
}

fn prepare_account_subcommand() -> Command {
    Command::new("account")
        .arg(prepare_data_dir())
        .about("Account management")
        .subcommand(Command::new("list").about("Lists accounts"))
        .subcommand(Command::new("ids").about("Lists accounts ids"))
        .subcommand(prepare_get_subcommand("Get account"))
}

fn prepare_customer_subcommand() -> Command {
    Command::new("customer")
        .arg(prepare_data_dir())
        .about("Customer management")
        .subcommand(Command::new("list").about("Lists customers"))
        .subcommand(Command::new("ids").about("Lists customers ids"))
        .subcommand(prepare_get_subcommand("Get customer"))
}

fn prepare_entry_subcommand() -> Command {
    Command::new("entry")
        .arg(prepare_data_dir())
        .about("Entry management")
        .subcommand(Command::new("list").about("Lists entries"))
        .subcommand(Command::new("ids").about("Lists entries ids"))
        .subcommand(prepare_get_subcommand("Get entry"))
        .subcommand(
            Command::new("create")
                .about("Create an entry")
                .arg(
                    Arg::new("id")
                        .help("New entry ID")
                        .short('I')
                        .long("id")
                        .num_args(1)
                        .required(true),
                )
                .arg(
                    Arg::new("name")
                        .help("New entry name")
                        .short('N')
                        .long("name")
                        .num_args(1)
                        .required(true),
                )
                .arg(
                    Arg::new("price")
                        .help("New entry price")
                        .short('P')
                        .long("price")
                        .num_args(1)
                        .required(true),
                )
                .arg(
                    Arg::new("currency")
                        .help("New entry currency")
                        .short('C')
                        .long("currency")
                        .num_args(1)
                        .required(true),
                )
                .arg(
                    Arg::new("details")
                        .help("New entry detail")
                        .short('D')
                        .long("detail")
                        .num_args(1..)
                        .required(false),
                )
                .arg(
                    Arg::new("git")
                        .env("UCELOFKA_GIT")
                        .help("Add newly created entry to git")
                        .short('G')
                        .long("git")
                        .num_args(0)
                        .required(false),
                ),
        )
}

fn prepare_identity_subcommand() -> Command {
    Command::new("identity")
        .arg(prepare_data_dir())
        .about("Identity management")
        .subcommand(Command::new("list").about("Lists identities"))
        .subcommand(Command::new("ids").about("Lists identities ids"))
        .subcommand(prepare_get_subcommand("Get identity"))
}

fn prepare_web() -> Command {
    Command::new("web")
        .arg(prepare_data_dir())
        .arg(
            Arg::new("port")
                .env("UCELOFKA_PORT")
                .help("Port which will be used for the web server")
                .long("port")
                .num_args(1)
                .value_parser(value_parser!(u16))
                .required(false)
                .default_value("8080"),
        )
        .about("start webserver frontend for ucelofka")
}

fn prepare_completions() -> Command {
    Command::new("completions")
        .about("completions generator")
        .arg(
            Arg::new("shell")
                .short('s')
                .long("shell")
                .help("For which shell the completion is supposed to be generated")
                .value_parser(["bash", "fish", "elvish", "powershell", "zsh"])
                .required(true),
        )
}

fn prepare_template_subcommand() -> Command {
    Command::new("template")
        .arg(prepare_data_dir())
        .about("Template management")
        .subcommand(Command::new("list").about("Lists templates"))
        .subcommand(prepare_get_subcommand("Get template"))
}

fn prepare_ids_subcommand() -> Command {
    Command::new("ids")
        .arg(prepare_data_dir())
        .about("Print ids of all entities")
}

fn prepare_cmd() -> Command {
    Command::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(prepare_invoice_subcommand())
        .subcommand(prepare_project_subcommand())
        .subcommand(prepare_account_subcommand())
        .subcommand(prepare_customer_subcommand())
        .subcommand(prepare_entry_subcommand())
        .subcommand(prepare_identity_subcommand())
        .subcommand(prepare_template_subcommand())
        .subcommand(prepare_web())
        .subcommand(prepare_completions())
        .subcommand(prepare_ids_subcommand())
}

fn get_data_dir(matches: &ArgMatches) -> Result<PathBuf> {
    let data_dir = matches.get_one::<PathBuf>("data_dir").unwrap();
    check_data_dir(data_dir)
}

fn exit_on_parse_error(mut cmd: Command) {
    println!();
    cmd.write_long_help(&mut io::stdout()).unwrap();
    std::process::exit(1);
}

fn process_invoice(cmd: Command, matches: &ArgMatches) -> Result<()> {
    let data_path = get_data_dir(matches)?;
    match matches.subcommand() {
        Some(("create", create_matches)) => {
            let due: Option<usize> = create_matches.get_one::<usize>("due").copied();
            let entries: Vec<String> = create_matches
                .get_many::<String>("entry")
                .unwrap()
                .map(String::from)
                .collect();

            let new_id = invoice::create(
                &data_path,
                &create_matches.get_one::<String>("customer").unwrap(),
                &create_matches.get_one::<String>("identity").unwrap(),
                &create_matches.get_one::<String>("account").unwrap(),
                entries,
                create_matches.get_flag("git"),
                due,
            )?;
            println!("Created invoice {}", new_id);
        }
        Some(("render", render_matches)) => {
            let invoice_id = render_matches
                .get_one::<String>("invoice")
                .unwrap()
                .to_string();
            let filename = invoice::render(
                data_path.as_ref(),
                &invoice_id,
                render_matches.get_one::<String>("template").unwrap(),
                render_matches.get_flag("git"),
            )?;
            println!(
                "{}",
                get_message(
                    "invoice-rendered",
                    Some(fluent_args!["filename" => filename, "invoice" => invoice_id])
                )
            );
        }
        Some(("list", _)) => {
            println!("{}", invoice::list(&data_path)?);
        }
        Some(("ids", _)) => {
            println!("{}", invoice::ids(&data_path)?);
        }
        Some(("get", get_matches)) => {
            let invoice_id = get_matches.get_one::<String>("id").unwrap();
            let invoice = invoice::get(&data_path, invoice_id)?;
            println!("{}", invoice);
        }
        _ => exit_on_parse_error(cmd),
    }
    Ok(())
}

fn process_project(cmd: Command, matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("make", make_matches)) => {
            project::make(
                make_matches.get_one::<PathBuf>("target").unwrap().as_path(),
                make_matches.get_flag("git"),
            )?;
        }
        _ => exit_on_parse_error(cmd),
    }
    Ok(())
}

fn process_accounts(cmd: Command, matches: &ArgMatches) -> Result<()> {
    let data_path = get_data_dir(matches)?;
    match matches.subcommand() {
        Some(("list", _)) => {
            println!("{}", account::list(&data_path)?);
        }
        Some(("ids", _)) => {
            println!("{}", account::ids(&data_path)?);
        }
        Some(("get", get_matches)) => {
            let account_id = get_matches.get_one::<String>("id").unwrap();
            let account = account::get(&data_path, account_id)?;
            println!("{}", account);
        }
        _ => exit_on_parse_error(cmd),
    }
    Ok(())
}

fn process_customer(cmd: Command, matches: &ArgMatches) -> Result<()> {
    let data_path = get_data_dir(matches)?;
    match matches.subcommand() {
        Some(("list", _)) => {
            println!("{}", customer::list(&data_path)?);
        }
        Some(("ids", _)) => {
            println!("{}", customer::ids(&data_path)?);
        }
        Some(("get", get_matches)) => {
            let customer_id = get_matches.get_one::<String>("id").unwrap();
            let customer = customer::get(&data_path, customer_id)?;
            println!("{}", customer);
        }
        _ => exit_on_parse_error(cmd),
    }
    Ok(())
}

fn process_entry(cmd: Command, matches: &ArgMatches) -> Result<()> {
    let data_path = get_data_dir(matches)?;
    match matches.subcommand() {
        Some(("list", _)) => {
            println!("{}", entry::list(&data_path)?);
        }
        Some(("ids", _)) => {
            println!("{}", entry::ids(&data_path)?);
        }
        Some(("get", get_matches)) => {
            let entry_id = get_matches.get_one::<String>("id").unwrap();
            let entry = entry::get(&data_path, entry_id)?;
            println!("{}", entry);
        }
        Some(("create", create_matches)) => {
            let id: String = create_matches.get_one::<String>("id").unwrap().to_string();
            let name: String = create_matches
                .get_one::<String>("name")
                .unwrap()
                .to_string();
            let price: f32 = create_matches
                .get_one::<String>("price")
                .unwrap()
                .parse()
                .unwrap();
            let currency: String = create_matches
                .get_one::<String>("currency")
                .unwrap()
                .to_string();
            let git = create_matches.get_flag("git");
            let details: Vec<String> = create_matches
                .get_many::<String>("details")
                .unwrap_or_default()
                .map(String::from)
                .collect();
            entry::create(&data_path, id, name, price, currency, details, git)?;
        }
        _ => exit_on_parse_error(cmd),
    }
    Ok(())
}

fn process_identity(cmd: Command, matches: &ArgMatches) -> Result<()> {
    let data_path = get_data_dir(matches)?;
    match matches.subcommand() {
        Some(("list", _)) => {
            println!("{}", identity::list(&data_path)?);
        }
        Some(("ids", _)) => {
            println!("{}", identity::ids(&data_path)?);
        }
        Some(("get", get_matches)) => {
            let identity_id = get_matches.get_one::<String>("id").unwrap();
            let identity = identity::get(&data_path, identity_id)?;
            println!("{}", identity);
        }
        _ => exit_on_parse_error(cmd),
    }
    Ok(())
}

fn process_template(cmd: Command, matches: &ArgMatches) -> Result<()> {
    let data_path = get_data_dir(matches)?;
    match matches.subcommand() {
        Some(("list", _)) => {
            println!("{}", template::list(&data_path)?);
        }
        Some(("get", get_matches)) => {
            let template_id = get_matches.get_one::<String>("id").unwrap();
            let template = template::get(&data_path, template_id)?;
            println!("{}", template);
        }
        _ => exit_on_parse_error(cmd),
    }
    Ok(())
}

fn process_web(_cmd: Command, matches: &ArgMatches) -> Result<()> {
    let data_path = get_data_dir(matches)?;
    let port: &u16 = matches.get_one("port").unwrap();
    web::run(*port, data_path)?;
    Ok(())
}

fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut io::stdout(),
    );
}

fn process_completions(mut cmd: Command, matches: &ArgMatches) -> Result<()> {
    let shell = matches.get_one::<String>("shell").unwrap();
    match shell.as_str() {
        "bash" => {
            print_completions(Bash, &mut cmd);
            Ok(())
        }
        "elvish" => {
            print_completions(Elvish, &mut cmd);
            Ok(())
        }
        "fish" => {
            print_completions(Fish, &mut cmd);
            Ok(())
        }
        "powershell" => {
            print_completions(PowerShell, &mut cmd);
            Ok(())
        }
        "zsh" => {
            print_completions(Zsh, &mut cmd);
            Ok(())
        }
        _ => unreachable!(),
    }
}

fn process_ids(_cmd: Command, matches: &ArgMatches) -> Result<()> {
    let data_path = get_data_dir(matches)?;
    println!("{}", ids::ids(&data_path)?);
    Ok(())
}

fn main() -> Result<()> {
    let cmd = prepare_cmd();

    let matches = cmd.clone().get_matches();

    match matches.subcommand() {
        Some(("invoice", invoice_matches)) => process_invoice(cmd.clone(), invoice_matches)?,
        Some(("project", project_matches)) => process_project(cmd.clone(), project_matches)?,
        Some(("account", account_matches)) => process_accounts(cmd.clone(), account_matches)?,
        Some(("customer", customer_matches)) => process_customer(cmd.clone(), customer_matches)?,
        Some(("entry", entry_matches)) => process_entry(cmd.clone(), entry_matches)?,
        Some(("identity", identity_matches)) => process_identity(cmd.clone(), identity_matches)?,
        Some(("template", identity_matches)) => process_template(cmd.clone(), identity_matches)?,
        Some(("web", web_matches)) => process_web(cmd.clone(), web_matches)?,
        Some(("completions", completions_matches)) => {
            process_completions(cmd.clone(), completions_matches)?
        }
        Some(("ids", ids_matches)) => process_ids(cmd.clone(), ids_matches)?,
        _ => exit_on_parse_error(cmd),
    }
    Ok(())
}
