pub mod actions;
pub mod data;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};
use std::io;
use std::path::Path;

use crate::actions::{invoice, project};

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

fn prepare_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("invoice")
                .arg(
                    Arg::with_name("data_dir")
                        .short("P")
                        .long("path")
                        .value_name("DATA_DIR")
                        .takes_value(true)
                        .required(false)
                        .validator(check_data_dir)
                        .help("path to data directory")
                        .default_value("."),
                )
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
                ),
        )
        .subcommand(
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
                ),
        )
}

fn main() {
    let mut app = prepare_app();

    let matches = app.clone().get_matches();

    let mut out = io::stdout();

    match matches.subcommand() {
        ("invoice", Some(invoice_matches)) => {
            let data_dir = invoice_matches.value_of("data_dir").unwrap();
            let data_path = Path::new(data_dir).canonicalize().unwrap();
            match invoice_matches.subcommand() {
                ("create", Some(_create_matches)) => {
                    println!("xx");
                    println!("xx");
                }
                ("render", Some(render_matches)) => {
                    invoice::render(
                        data_path.as_ref(),
                        render_matches.value_of("invoice").unwrap(),
                        render_matches.value_of("template").unwrap(),
                    );
                }
                ("list", Some(_)) => {
                    invoice::list(data_path.as_ref());
                }
                _ => {
                    app.write_long_help(&mut out).unwrap();
                    println!();
                    return;
                }
            }
        }
        ("project", Some(project_matches)) => match project_matches.subcommand() {
            ("make", Some(new_matches)) => {
                project::make(new_matches.value_of("target").unwrap());
            }
            _ => {
                app.write_long_help(&mut out).unwrap();
                println!();
                return;
            }
        },
        _ => {
            app.write_long_help(&mut out).unwrap();
            println!();
            return;
        }
    }
}
