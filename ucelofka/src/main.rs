pub mod actions;
pub mod storage;
pub mod translations;
pub mod web;

use anyhow::{anyhow, Result};
use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches, Values,
};
use clap_generate::generators::{Bash, Elvish, Fish, PowerShell, Zsh};
use clap_generate::{generate, Generator};
use fluent::fluent_args;
use std::io;
use std::path::{Path, PathBuf};
use ucelofka_data as data;

use crate::{
    actions::{account, customer, entry, identity, ids, invoice, project, template},
    translations::{get_message, texts},
};

pub fn check_data_dir(path_str: &str) -> Result<()> {
    let root_dir: &Path = Path::new(&path_str);
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
        Ok(())
    } else {
        Err(anyhow!(get_message(
            "path-not-exits",
            Some(fluent_args!["path" => path_str])
        )))
    }
}

fn prepare_data_dir() -> Arg<'static> {
    Arg::new("data_dir")
        .short('P')
        .long("path")
        .value_name("DATA_DIR")
        .takes_value(true)
        .required(false)
        .validator(|s| check_data_dir(s).map_err(|e| e.to_string()))
        .about(&texts::DATA_DIRECTORY_PATH)
        .default_value(".")
}

fn prepare_get_subcommand(about: &'static str) -> App<'static> {
    App::new("get")
        .arg(
            Arg::new("id")
                .short('I')
                .long("id")
                .takes_value(true)
                .required(true),
        )
        .about(about)
}

fn prepare_invoice_subcommand() -> App<'static> {
    App::new("invoice")
        .arg(prepare_data_dir())
        .about("Invoice management")
        .subcommand(
            App::new("create")
                .about("Creates a new invoice")
                .arg(
                    Arg::new("customer")
                        .about("Customer id")
                        .long("customer")
                        .short('C')
                        .multiple(false)
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("identity")
                        .about("Identity id")
                        .short('I')
                        .long("identity")
                        .multiple(false)
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("account")
                        .about("Account id")
                        .long("account")
                        .short('A')
                        .multiple(false)
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("entry")
                        .about("Entry id")
                        .short('E')
                        .long("entry")
                        .multiple(true)
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("git")
                        .env("UCELOFKA_GIT")
                        .about("Add newly created invoice to git")
                        .short('G')
                        .long("git")
                        .takes_value(false)
                        .required(false),
                )
                .arg(
                    Arg::new("due")
                        .about("Due time (in days)")
                        .short('D')
                        .long("due")
                        .takes_value(true)
                        .required(false)
                        .validator(|v: &str| {
                            v.parse::<usize>().map(|_| ()).map_err(|e| e.to_string())
                        }),
                ),
        )
        .subcommand(App::new("list").about("Lists invoices"))
        .subcommand(App::new("ids").about("List invoice ids"))
        .subcommand(
            App::new("render")
                .about("Renders invoice")
                .arg(
                    Arg::new("template")
                        .about("Template id")
                        .short('T')
                        .long("template")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("invoice")
                        .about("Invoice id")
                        .short('I')
                        .long("invoice")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("git")
                        .env("UCELOFKA_GIT")
                        .about("Add newly created file to git")
                        .short('G')
                        .long("git")
                        .takes_value(false)
                        .required(false),
                ),
        )
        .subcommand(prepare_get_subcommand("Get invoice"))
}

fn prepare_project_subcommand() -> App<'static> {
    App::new("project")
        .about("Manages data project")
        .subcommand(
            App::new("make")
                .about("Creates new data dir")
                .arg(
                    Arg::new("target")
                        .about("Where is should be placed")
                        .short('T')
                        .long("target")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("git")
                        .env("UCELOFKA_GIT")
                        .about("Initialize with a git repository")
                        .short('G')
                        .long("git")
                        .takes_value(false)
                        .required(false),
                ),
        )
}

fn prepare_account_subcommand() -> App<'static> {
    App::new("account")
        .arg(prepare_data_dir())
        .about("Account management")
        .subcommand(App::new("list").about("Lists accounts"))
        .subcommand(App::new("ids").about("Lists accounts ids"))
        .subcommand(prepare_get_subcommand("Get account"))
}

fn prepare_customer_subcommand() -> App<'static> {
    App::new("customer")
        .arg(prepare_data_dir())
        .about("Customer management")
        .subcommand(App::new("list").about("Lists customers"))
        .subcommand(App::new("ids").about("Lists customers ids"))
        .subcommand(prepare_get_subcommand("Get customer"))
}

fn prepare_entry_subcommand() -> App<'static> {
    App::new("entry")
        .arg(prepare_data_dir())
        .about("Entry management")
        .subcommand(App::new("list").about("Lists entries"))
        .subcommand(App::new("ids").about("Lists entries ids"))
        .subcommand(prepare_get_subcommand("Get entry"))
        .subcommand(
            App::new("create")
                .about("Create an entry")
                .arg(
                    Arg::new("id")
                        .about("New entry ID")
                        .short('I')
                        .long("id")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("name")
                        .about("New entry name")
                        .short('N')
                        .long("name")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("price")
                        .about("New entry price")
                        .short('P')
                        .long("price")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("currency")
                        .about("New entry currency")
                        .short('C')
                        .long("currency")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("details")
                        .about("New entry detail")
                        .short('D')
                        .long("detail")
                        .takes_value(true)
                        .multiple(true)
                        .required(false),
                )
                .arg(
                    Arg::new("git")
                        .env("UCELOFKA_GIT")
                        .about("Add newly created entry to git")
                        .short('G')
                        .long("git")
                        .takes_value(false)
                        .required(false),
                ),
        )
}

fn prepare_identity_subcommand() -> App<'static> {
    App::new("identity")
        .arg(prepare_data_dir())
        .about("Identity management")
        .subcommand(App::new("list").about("Lists identities"))
        .subcommand(App::new("ids").about("Lists identities ids"))
        .subcommand(prepare_get_subcommand("Get identity"))
}

fn prepare_web() -> App<'static> {
    App::new("web")
        .arg(prepare_data_dir())
        .arg(
            Arg::new("port")
                .env("UCELOFKA_PORT")
                .about("Port which will be used for the web server")
                .long("port")
                .takes_value(true)
                .required(false)
                .default_value("8080"),
        )
        .about("start webserver frontend for ucelofka")
}

fn prepare_completions() -> App<'static> {
    App::new("completions").about("completions generator").arg(
        Arg::new("shell")
            .short('s')
            .long("shell")
            .about("For which shell the completion is supposed to be generated")
            .possible_values(&["bash", "fish", "elvish", "powershell", "zsh"])
            .required(true),
    )
}

fn prepare_template_subcommand() -> App<'static> {
    App::new("template")
        .arg(prepare_data_dir())
        .about("Template management")
        .subcommand(App::new("list").about("Lists templates"))
        .subcommand(prepare_get_subcommand("Get template"))
}

fn prepare_ids_subcommand() -> App<'static> {
    App::new("ids")
        .arg(prepare_data_dir())
        .about("Print ids of all entities")
}

fn prepare_app() -> App<'static> {
    App::new(crate_name!())
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

fn get_data_dir(matches: &ArgMatches) -> PathBuf {
    let data_dir = matches.value_of("data_dir").unwrap();
    Path::new(data_dir).canonicalize().unwrap()
}

fn exit_on_parse_error(mut app: App) {
    println!();
    app.write_long_help(&mut io::stdout()).unwrap();
    std::process::exit(1);
}

fn process_invoice(app: App, matches: &ArgMatches) -> Result<()> {
    let data_path = get_data_dir(matches);
    match matches.subcommand() {
        Some(("create", create_matches)) => {
            let due: Option<usize> = match create_matches.value_of("due") {
                Some(due_str) => Some(due_str.parse().unwrap()),
                None => None,
            };

            let new_id = invoice::create(
                &data_path,
                create_matches.value_of("customer").unwrap(),
                create_matches.value_of("identity").unwrap(),
                create_matches.value_of("account").unwrap(),
                create_matches.values_of("entry").unwrap().collect(),
                create_matches.is_present("git"),
                due,
            )?;
            println!("Created invoice {}", new_id);
        }
        Some(("render", render_matches)) => {
            let invoice_id = render_matches.value_of("invoice").unwrap();
            let filename = invoice::render(
                data_path.as_ref(),
                invoice_id,
                render_matches.value_of("template").unwrap(),
                render_matches.is_present("git"),
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
            let invoice_id = get_matches.value_of("id").unwrap();
            let invoice = invoice::get(&data_path, invoice_id)?;
            println!("{}", invoice);
        }
        _ => exit_on_parse_error(app),
    }
    Ok(())
}

fn process_project(app: App, matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("make", make_matches)) => {
            project::make(
                make_matches.value_of("target").unwrap(),
                make_matches.is_present("git"),
            )?;
        }
        _ => exit_on_parse_error(app),
    }
    Ok(())
}

fn process_accounts(app: App, matches: &ArgMatches) -> Result<()> {
    let data_path = get_data_dir(matches);
    match matches.subcommand() {
        Some(("list", _)) => {
            println!("{}", account::list(&data_path)?);
        }
        Some(("ids", _)) => {
            println!("{}", account::ids(&data_path)?);
        }
        Some(("get", get_matches)) => {
            let account_id = get_matches.value_of("id").unwrap();
            let account = account::get(&data_path, account_id)?;
            println!("{}", account);
        }
        _ => exit_on_parse_error(app),
    }
    Ok(())
}

fn process_customer(app: App, matches: &ArgMatches) -> Result<()> {
    let data_path = get_data_dir(matches);
    match matches.subcommand() {
        Some(("list", _)) => {
            println!("{}", customer::list(&data_path)?);
        }
        Some(("ids", _)) => {
            println!("{}", customer::ids(&data_path)?);
        }
        Some(("get", get_matches)) => {
            let customer_id = get_matches.value_of("id").unwrap();
            let customer = customer::get(&data_path, customer_id)?;
            println!("{}", customer);
        }
        _ => exit_on_parse_error(app),
    }
    Ok(())
}

fn process_entry(app: App, matches: &ArgMatches) -> Result<()> {
    let data_path = get_data_dir(matches);
    match matches.subcommand() {
        Some(("list", _)) => {
            println!("{}", entry::list(&data_path)?);
        }
        Some(("ids", _)) => {
            println!("{}", entry::ids(&data_path)?);
        }
        Some(("get", get_matches)) => {
            let entry_id = get_matches.value_of("id").unwrap();
            let entry = entry::get(&data_path, entry_id)?;
            println!("{}", entry);
        }
        Some(("create", create_matches)) => {
            let id: String = create_matches.value_of("id").unwrap().to_string();
            let name: String = create_matches.value_of("name").unwrap().to_string();
            let price: f32 = create_matches.value_of("price").unwrap().parse().unwrap();
            let currency: String = create_matches.value_of("currency").unwrap().to_string();
            let git = create_matches.is_present("git");
            let details: Vec<String> = create_matches
                .values_of("details")
                .or_else(|| Some(Values::default()))
                .unwrap()
                .map(String::from)
                .collect();
            entry::create(&data_path, id, name, price, currency, details, git)?;
        }
        _ => exit_on_parse_error(app),
    }
    Ok(())
}

fn process_identity(app: App, matches: &ArgMatches) -> Result<()> {
    let data_path = get_data_dir(matches);
    match matches.subcommand() {
        Some(("list", _)) => {
            println!("{}", identity::list(&data_path)?);
        }
        Some(("ids", _)) => {
            println!("{}", identity::ids(&data_path)?);
        }
        Some(("get", get_matches)) => {
            let identity_id = get_matches.value_of("id").unwrap();
            let identity = identity::get(&data_path, identity_id)?;
            println!("{}", identity);
        }
        _ => exit_on_parse_error(app),
    }
    Ok(())
}

fn process_template(app: App, matches: &ArgMatches) -> Result<()> {
    let data_path = get_data_dir(matches);
    match matches.subcommand() {
        Some(("list", _)) => {
            println!("{}", template::list(&data_path)?);
        }
        Some(("get", get_matches)) => {
            let template_id = get_matches.value_of("id").unwrap();
            let template = template::get(&data_path, template_id)?;
            println!("{}", template);
        }
        _ => exit_on_parse_error(app),
    }
    Ok(())
}

fn process_web(_app: App, matches: &ArgMatches) -> Result<()> {
    let data_path = get_data_dir(matches);
    let port_str = matches.value_of("port").unwrap();
    let port: u16 = port_str.parse::<u16>().map_err(|_| {
        anyhow!(get_message(
            "not-a-port-number",
            Some(fluent_args!["port" => port_str])
        ))
    })?;
    web::run(port, data_path)?;
    Ok(())
}

fn print_completions<G: Generator>(app: &mut App) {
    generate::<G, _>(app, app.get_name().to_string(), &mut io::stdout());
}

fn process_completions(mut app: App, matches: &ArgMatches) -> Result<()> {
    let shell = matches.value_of("shell").unwrap();
    match shell {
        "bash" => {
            print_completions::<Bash>(&mut app);
            Ok(())
        }
        "elvish" => {
            print_completions::<Elvish>(&mut app);
            Ok(())
        }
        "fish" => {
            print_completions::<Fish>(&mut app);
            Ok(())
        }
        "powershell" => {
            print_completions::<PowerShell>(&mut app);
            Ok(())
        }
        "zsh" => {
            print_completions::<Zsh>(&mut app);
            Ok(())
        }
        _ => unreachable!(),
    }
}

fn process_ids(_app: App, matches: &ArgMatches) -> Result<()> {
    let data_path = get_data_dir(matches);
    println!("{}", ids::ids(&data_path)?);
    Ok(())
}

fn main() -> Result<()> {
    let app = prepare_app();

    let matches = app.clone().get_matches();

    match matches.subcommand() {
        Some(("invoice", invoice_matches)) => process_invoice(app.clone(), invoice_matches)?,
        Some(("project", project_matches)) => process_project(app.clone(), project_matches)?,
        Some(("account", account_matches)) => process_accounts(app.clone(), account_matches)?,
        Some(("customer", customer_matches)) => process_customer(app.clone(), customer_matches)?,
        Some(("entry", entry_matches)) => process_entry(app.clone(), entry_matches)?,
        Some(("identity", identity_matches)) => process_identity(app.clone(), identity_matches)?,
        Some(("template", identity_matches)) => process_template(app.clone(), identity_matches)?,
        Some(("web", web_matches)) => process_web(app.clone(), web_matches)?,
        Some(("completions", completions_matches)) => {
            process_completions(app.clone(), completions_matches)?
        }
        Some(("ids", ids_matches)) => process_ids(app.clone(), ids_matches)?,
        _ => exit_on_parse_error(app),
    }
    Ok(())
}
