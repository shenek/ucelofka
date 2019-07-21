pub mod actions;
pub mod data;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};
use std::io;
use std::path::Path;

use crate::actions::{account, customer, entry, identity, invoice, project};

pub fn check_data_dir(path_str: String) -> Result<(), String> {
    let root_dir: &Path = Path::new(&path_str[..]);
    if let Ok(path) = root_dir.canonicalize() {
        if !path.is_dir() {
            return Err(format!("{} is not directory", path_str));
        }
        for subdir in &[
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
                return Err(format!(
                    "data directory {} is missing {} subdir",
                    root_dir.to_str().unwrap_or("?"),
                    subdir
                ));
            }
        }
        Ok(())
    } else {
        Err(format!("{} path does not exist", path_str))
    }
}

fn prepare_data_dir() -> Arg<'static, 'static> {
    Arg::with_name("data_dir")
        .short("P")
        .long("path")
        .value_name("DATA_DIR")
        .takes_value(true)
        .required(false)
        .validator(check_data_dir)
        .help("path to data directory")
        .default_value(".")
}

fn prepare_get_subcommand(about: &'static str) -> App<'static, 'static> {
    SubCommand::with_name("get")
        .arg(
            Arg::with_name("id")
                .short("I")
                .long("id")
                .takes_value(true)
                .required(true),
        )
        .about(about)
}

fn prepare_invoice_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("invoice")
        .arg(prepare_data_dir())
        .about("Invoice management")
        .subcommand(
            SubCommand::with_name("create")
                .about("Creates a new invoice")
                .arg(
                    Arg::with_name("customer")
                        .help("Customer id")
                        .long("customer")
                        .short("C")
                        .multiple(false)
                        .required(true),
                )
                .arg(
                    Arg::with_name("identity")
                        .help("Identity id")
                        .short("I")
                        .long("identity")
                        .multiple(false)
                        .required(true),
                )
                .arg(
                    Arg::with_name("account")
                        .help("Account id")
                        .long("acount")
                        .short("A")
                        .multiple(false)
                        .required(true),
                )
                .arg(
                    Arg::with_name("entry")
                        .help("Entry id")
                        .short("E")
                        .long("entry")
                        .multiple(true)
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("Lists invoices"))
        .subcommand(
            SubCommand::with_name("render")
                .about("Renders invoice")
                .arg(
                    Arg::with_name("template")
                        .help("Template id")
                        .short("T")
                        .long("template")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("invoice")
                        .help("Invoice id")
                        .short("I")
                        .long("invoice")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(prepare_get_subcommand("Get invoice"))
}

fn prepare_project_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("project")
        .about("Manages data project")
        .subcommand(
            SubCommand::with_name("make")
                .about("Creates new data dir")
                .arg(
                    Arg::with_name("target")
                        .help("Where is should be placed")
                        .short("T")
                        .long("target")
                        .takes_value(true)
                        .required(true),
                ),
        )
}

fn prepare_account_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("account")
        .arg(prepare_data_dir())
        .about("Account management")
        .subcommand(SubCommand::with_name("list").about("Lists account"))
        .subcommand(prepare_get_subcommand("Get account"))
}

fn prepare_customer_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("customer")
        .arg(prepare_data_dir())
        .about("Customer management")
        .subcommand(SubCommand::with_name("list").about("Lists customers"))
        .subcommand(prepare_get_subcommand("Get customer"))
}

fn prepare_entry_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("entry")
        .arg(prepare_data_dir())
        .about("Entry management")
        .subcommand(SubCommand::with_name("list").about("Lists entries"))
        .subcommand(prepare_get_subcommand("Get entry"))
}

fn prepare_identity_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("identity")
        .arg(prepare_data_dir())
        .about("Identity management")
        .subcommand(SubCommand::with_name("list").about("Lists identities"))
        .subcommand(prepare_get_subcommand("Get identity"))
}

fn prepare_app() -> App<'static, 'static> {
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
}

fn main() {
    let mut app = prepare_app();
    let mut out = io::stdout();

    let matches = app.clone().get_matches();

    match matches.subcommand() {
        ("invoice", Some(invoice_matches)) => {
            let data_dir = invoice_matches.value_of("data_dir").unwrap();
            let data_path = Path::new(data_dir).canonicalize().unwrap();
            match invoice_matches.subcommand() {
                ("create", Some(create_matches)) => invoice::create(
                    &data_path,
                    create_matches.value_of("identity").unwrap(),
                    create_matches.value_of("customer").unwrap(),
                    create_matches.value_of("account").unwrap(),
                    create_matches.values_of("entry").unwrap().collect(),
                ),
                ("render", Some(render_matches)) => {
                    invoice::render(
                        data_path.as_ref(),
                        render_matches.value_of("invoice").unwrap(),
                        render_matches.value_of("template").unwrap(),
                    );
                }
                ("list", Some(_)) => {
                    println!("{}", invoice::list(&data_path));
                }
                ("get", Some(get_matches)) => {
                    if let Some(invoice) = invoice::get(
                        &data_path,
                        get_matches
                            .value_of("id")
                            .unwrap()
                            .parse::<u64>()
                            .unwrap_or_else(|_| std::process::exit(1)),
                    ) {
                        println!("{}", invoice);
                        return;
                    } else {
                        std::process::exit(1);
                    }
                }
                _ => {
                    app.write_long_help(&mut out).unwrap();
                    println!();
                    return;
                }
            }
        }
        ("project", Some(project_matches)) => match project_matches.subcommand() {
            ("make", Some(make_matches)) => {
                project::make(make_matches.value_of("target").unwrap());
            }
            _ => {
                app.write_long_help(&mut out).unwrap();
                println!();
                return;
            }
        },
        ("account", Some(account_matches)) => {
            let data_dir = account_matches.value_of("data_dir").unwrap();
            let data_path = Path::new(data_dir).canonicalize().unwrap();
            match account_matches.subcommand() {
                ("list", Some(_)) => {
                    println!("{}", account::list(&data_path));
                }
                ("get", Some(get_matches)) => {
                    if let Some(account) =
                        account::get(&data_path, get_matches.value_of("id").unwrap())
                    {
                        println!("{}", account);
                        return;
                    } else {
                        std::process::exit(1);
                    }
                }
                _ => {
                    app.write_long_help(&mut out).unwrap();
                    println!();
                    return;
                }
            }
        }
        ("customer", Some(customer_matches)) => {
            let data_dir = customer_matches.value_of("data_dir").unwrap();
            let data_path = Path::new(data_dir).canonicalize().unwrap();
            match customer_matches.subcommand() {
                ("list", Some(_)) => {
                    println!("{}", customer::list(&data_path));
                }
                ("get", Some(get_matches)) => {
                    if let Some(customer) =
                        customer::get(&data_path, get_matches.value_of("id").unwrap())
                    {
                        println!("{}", customer);
                        return;
                    } else {
                        std::process::exit(1);
                    }
                }
                _ => {
                    app.write_long_help(&mut out).unwrap();
                    println!();
                    return;
                }
            }
        }
        ("entry", Some(entry_matches)) => {
            let data_dir = entry_matches.value_of("data_dir").unwrap();
            let data_path = Path::new(data_dir).canonicalize().unwrap();
            match entry_matches.subcommand() {
                ("list", Some(_)) => {
                    println!("{}", entry::list(&data_path));
                }
                ("get", Some(get_matches)) => {
                    if let Some(entry) = entry::get(&data_path, get_matches.value_of("id").unwrap())
                    {
                        println!("{}", entry);
                        return;
                    } else {
                        std::process::exit(1);
                    }
                }
                _ => {
                    app.write_long_help(&mut out).unwrap();
                    println!();
                    return;
                }
            }
        }
        ("identity", Some(identity_matches)) => {
            let data_dir = identity_matches.value_of("data_dir").unwrap();
            let data_path = Path::new(data_dir).canonicalize().unwrap();
            match identity_matches.subcommand() {
                ("list", Some(_)) => {
                    println!("{}", identity::list(&data_path));
                }
                ("get", Some(get_matches)) => {
                    if let Some(identity) =
                        identity::get(&data_path, get_matches.value_of("id").unwrap())
                    {
                        println!("{}", identity);
                        return;
                    } else {
                        std::process::exit(1);
                    }
                }
                _ => {
                    app.write_long_help(&mut out).unwrap();
                    println!();
                    return;
                }
            }
        }
        _ => {
            app.write_long_help(&mut out).unwrap();
            println!();
            return;
        }
    }
}
