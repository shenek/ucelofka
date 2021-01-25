#![allow(dead_code)]

use anyhow::Error;
use ucelofka_data::{
    account::Account, customer::Customer, entry::Entry, identity::Identity, invoice::Invoice,
    template::Template,
};
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

use crate::components::page;

#[derive(PartialEq, Clone)]
pub enum MenuItem {
    Identities,
    Accounts,
    Customers,
    Entries,
    Invoices,
}

impl Default for MenuItem {
    fn default() -> Self {
        MenuItem::Identities
    }
}

pub enum Messages {
    GetIdentities,
    GetAccounts,
    GetCustomers,
    GetEntries,
    GetInvoices,
    GetTemplates,
    SelectMenuItem(MenuItem),
    UpdateIdentities(Result<Vec<Identity>, Error>),
    UpdateAccounts(Result<Vec<Account>, Error>),
    UpdateCustomers(Result<Vec<Customer>, Error>),
    UpdateEntries(Result<Vec<Entry>, Error>),
    UpdateInvoices(Result<Vec<Invoice>, Error>),
    UpdateTemplates(Result<Vec<Template>, Error>),
}

#[derive(Default, Properties, Clone, PartialEq)]
#[allow(dead_code)]
pub struct AppProps {
    #[prop_or_default]
    identities: Vec<Identity>,
    #[prop_or_default]
    accounts: Vec<Account>,
    #[prop_or_default]
    customers: Vec<Customer>,
    #[prop_or_default]
    entries: Vec<Entry>,
    #[prop_or_default]
    invoices: Vec<Invoice>,
    #[prop_or_default]
    templates: Vec<Template>,
    #[prop_or_default]
    error: Option<String>,
    #[prop_or_default]
    selected_menu_item: MenuItem,
}

pub struct App {
    props: AppProps,
    link: ComponentLink<Self>,
    fetch_identities_task: Option<FetchTask>,
    fetch_accounts_task: Option<FetchTask>,
    fetch_customers_task: Option<FetchTask>,
    fetch_entries_task: Option<FetchTask>,
    fetch_invoices_task: Option<FetchTask>,
    fetch_templates_task: Option<FetchTask>,
}

impl App {
    fn page(&self) -> Html {
        match self.props.selected_menu_item {
            MenuItem::Identities => {
                html! {
                    <>
                        <page::Identities identities=self.props.identities.clone()></page::Identities>
                    </>
                }
            }
            MenuItem::Accounts => {
                html! {
                    <>
                        <page::Accounts accounts=self.props.accounts.clone()></page::Accounts>
                    </>
                }
            }
            MenuItem::Customers => {
                html! {
                    <>
                        <page::Customers customers=self.props.customers.clone()></page::Customers>
                    </>
                }
            }
            MenuItem::Entries => {
                html! {
                    <>
                        <page::Entries entries=self.props.entries.clone()></page::Entries>
                    </>
                }
            }
            MenuItem::Invoices => {
                html! {
                    <>
                        <page::Invoices invoices=self.props.invoices.clone()></page::Invoices>
                    </>
                }
            }
        }
    }
}

impl Component for App {
    type Message = Messages;
    type Properties = AppProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message_batch(vec![
            Messages::GetIdentities,
            Messages::GetAccounts,
            Messages::GetCustomers,
            Messages::GetEntries,
            Messages::GetInvoices,
            Messages::GetTemplates,
        ]);
        Self {
            props,
            link,
            fetch_identities_task: None,
            fetch_accounts_task: None,
            fetch_customers_task: None,
            fetch_entries_task: None,
            fetch_invoices_task: None,
            fetch_templates_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::UpdateIdentities(Ok(identities)) => {
                self.props.identities = identities;
                self.fetch_identities_task = None;
                true
            }
            Messages::UpdateAccounts(Ok(accounts)) => {
                self.props.accounts = accounts;
                self.fetch_accounts_task = None;
                true
            }
            Messages::UpdateCustomers(Ok(customers)) => {
                self.props.customers = customers;
                self.fetch_customers_task = None;
                true
            }
            Messages::UpdateEntries(Ok(entries)) => {
                self.props.entries = entries;
                self.fetch_entries_task = None;
                true
            }
            Messages::UpdateInvoices(Ok(invoices)) => {
                self.props.invoices = invoices;
                self.fetch_invoices_task = None;
                true
            }
            Messages::UpdateTemplates(Ok(templates)) => {
                self.props.templates = templates;
                self.fetch_templates_task = None;
                true
            }
            Messages::UpdateIdentities(Err(error))
            | Messages::UpdateAccounts(Err(error))
            | Messages::UpdateCustomers(Err(error))
            | Messages::UpdateEntries(Err(error))
            | Messages::UpdateInvoices(Err(error))
            | Messages::UpdateTemplates(Err(error)) => {
                self.props.error = Some(error.to_string());
                true
            }
            Messages::GetIdentities => {
                if self.fetch_identities_task.is_none() {
                    let request = Request::get("/api/identity/").body(Nothing).unwrap();
                    let callback = self.link.callback(
                        |response: Response<Json<Result<Vec<Identity>, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Messages::UpdateIdentities(data)
                        },
                    );
                    let task = FetchService::fetch(request, callback).unwrap();
                    self.fetch_identities_task = Some(task);
                }
                false
            }
            Messages::GetAccounts => {
                if self.fetch_accounts_task.is_none() {
                    let request = Request::get("/api/account/").body(Nothing).unwrap();
                    let callback = self.link.callback(
                        |response: Response<Json<Result<Vec<Account>, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Messages::UpdateAccounts(data)
                        },
                    );
                    let task = FetchService::fetch(request, callback).unwrap();
                    self.fetch_accounts_task = Some(task);
                }
                false
            }
            Messages::GetCustomers => {
                if self.fetch_customers_task.is_none() {
                    let request = Request::get("/api/customer/").body(Nothing).unwrap();
                    let callback = self.link.callback(
                        |response: Response<Json<Result<Vec<Customer>, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Messages::UpdateCustomers(data)
                        },
                    );
                    let task = FetchService::fetch(request, callback).unwrap();
                    self.fetch_customers_task = Some(task);
                }
                false
            }
            Messages::GetEntries => {
                if self.fetch_entries_task.is_none() {
                    let request = Request::get("/api/entry/").body(Nothing).unwrap();
                    let callback = self.link.callback(
                        |response: Response<Json<Result<Vec<Entry>, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Messages::UpdateEntries(data)
                        },
                    );
                    let task = FetchService::fetch(request, callback).unwrap();
                    self.fetch_entries_task = Some(task);
                }
                false
            }
            Messages::GetInvoices => {
                if self.fetch_invoices_task.is_none() {
                    let request = Request::get("/api/invoice/").body(Nothing).unwrap();
                    let callback = self.link.callback(
                        |response: Response<Json<Result<Vec<Invoice>, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Messages::UpdateInvoices(data)
                        },
                    );
                    let task = FetchService::fetch(request, callback).unwrap();
                    self.fetch_invoices_task = Some(task);
                }
                false
            }
            Messages::GetTemplates => {
                if self.fetch_templates_task.is_none() {
                    let request = Request::get("/api/template/").body(Nothing).unwrap();
                    let callback = self.link.callback(
                        |response: Response<Json<Result<Vec<Template>, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Messages::UpdateTemplates(data)
                        },
                    );
                    let task = FetchService::fetch(request, callback).unwrap();
                    self.fetch_templates_task = Some(task);
                }
                false
            }
            Messages::SelectMenuItem(item) => {
                if self.props.selected_menu_item == item {
                    false
                } else {
                    self.props.selected_menu_item = item;
                    true
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let menu_items = html! {
            <>
                <li>
                    <a
                        class=if self.props.selected_menu_item == MenuItem::Identities {"is-active"} else {""}
                        onclick=self.link.callback(|_| Messages::SelectMenuItem(MenuItem::Identities))
                    >
                        {"Identities"}<ybc::Tag classes="is-pulled-right">{self.props.identities.len()}</ybc::Tag>
                    </a>
                </li>
                <li>
                    <a
                        class=if self.props.selected_menu_item == MenuItem::Accounts {"is-active"} else {""}
                        onclick=self.link.callback(|_| Messages::SelectMenuItem(MenuItem::Accounts))
                    >
                        {"Accounts"}<ybc::Tag classes="is-pulled-right">{self.props.accounts.len()}</ybc::Tag>
                    </a>
                </li>
                <li><a
                        class=if self.props.selected_menu_item == MenuItem::Customers {"is-active"} else {""}
                        onclick=self.link.callback(|_| Messages::SelectMenuItem(MenuItem::Customers))
                    >
                        {"Customers"}<ybc::Tag classes="is-pulled-right">{self.props.customers.len()}</ybc::Tag>
                    </a>
                </li>
                <li>
                    <a
                        class=if self.props.selected_menu_item == MenuItem::Entries {"is-active"} else {""}
                        onclick=self.link.callback(|_| Messages::SelectMenuItem(MenuItem::Entries))
                    >
                        {"Entries"}<ybc::Tag classes="is-pulled-right">{self.props.entries.len()}</ybc::Tag>
                    </a>
                </li>
                <li>
                    <a
                        class=if self.props.selected_menu_item == MenuItem::Invoices {"is-active"} else {""}
                        onclick=self.link.callback(|_| Messages::SelectMenuItem(MenuItem::Invoices))
                    >
                        {"Invoices"}<ybc::Tag classes="is-pulled-right">{self.props.invoices.len()}</ybc::Tag>
                    </a>
                </li>
            </>
        };
        let hero_body = html! {
            <>
                <ybc::Container classes="has-text-centered">
                    <ybc::Title>{"Ucelofka"}</ybc::Title>
                    <ybc::Subtitle>{"Manage your invoices in GIT"}</ybc::Subtitle>
                </ybc::Container>
            </>
        };
        html! {
            <>
                <ybc::Hero body={hero_body} classes="is-light"></ybc::Hero>
                <ybc::Section>
                    <ybc::Container>
                        <ybc::Columns>
                            <ybc::Column classes="is-3">
                                <ybc::Menu>
                                    <ybc::MenuLabel text="Entities">
                                    </ybc::MenuLabel>
                                    <ybc::MenuList>
                                        { menu_items }
                                    </ybc::MenuList>
                                </ybc::Menu>
                            </ybc::Column>
                            <ybc::Column classes="is-9">
                                { self.page() }
                            </ybc::Column>
                        </ybc::Columns>
                    </ybc::Container>
                </ybc::Section>
                <ybc::Footer>
                    <ybc::Content classes="has-text-centered">
                        <p>
                            <strong>{"Ucelofka"}</strong>
                        </p>
                    </ybc::Content>
                </ybc::Footer>
            </>
        }
    }
}
