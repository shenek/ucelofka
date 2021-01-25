use ucelofka_data::account::Account;
use yew::prelude::*;

pub enum Messages {
    SelectAccount(usize),
}

#[derive(Default, Properties, Clone, PartialEq)]
pub struct AccountsProps {
    #[prop_or_default]
    pub accounts: Vec<Account>,
    #[prop_or_default]
    pub selected_idx: Option<usize>,
}

pub struct Accounts {
    props: AccountsProps,
    link: ComponentLink<Self>,
}

impl Component for Accounts {
    type Message = Messages;
    type Properties = AccountsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::SelectAccount(idx) => {
                if let Some(selected_idx) = self.props.selected_idx {
                    if selected_idx == idx {
                        // Deselect
                        self.props.selected_idx = None;
                        return true;
                    }
                }
                self.props.selected_idx = Some(idx);
            }
        }
        true
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
        html! {
            <ybc::Container>
                <ybc::Panel heading=html!{{"Accounts"}}>
                {
                    for self.props.accounts.iter().enumerate().map(|(idx, identity)| {
                        html! {
                            <ybc::PanelBlock
                                tag="a"
                                onclick=self.link.callback(move |_| Messages::SelectAccount(idx))
                                active=Some(idx) == self.props.selected_idx
                            >
                                {&identity.id}
                            </ybc::PanelBlock>
                        }
                    })
                }
                </ybc::Panel>
                {
                    if let Some(idx) = self.props.selected_idx {
                        html!{
                            <ybc::Card>
                                <ybc::CardHeader>
                                    <p class="card-header-title">{&self.props.accounts[idx].id}</p>
                                </ybc::CardHeader>
                                <ybc::CardContent>
                                    <ybc::Content>
                                    <dl>
                                        <dt><span class="has-text-weight-bold">{"Name:"}</span></dt><dd>{&self.props.accounts[idx].name}</dd>
                                        <dt><span class="has-text-weight-bold">{"Bank name:"}</span></dt><dd>{&self.props.accounts[idx].bank_name}</dd>
                                        <dt><span class="has-text-weight-bold">{"Account name:"}</span></dt><dd>{&self.props.accounts[idx].account_name}</dd>
                                        <dt><span class="has-text-weight-bold">{"Account number:"}</span></dt><dd>{&self.props.accounts[idx].account_number}</dd>
                                        <dt><span class="has-text-weight-bold">{"IBAN:"}</span></dt><dd>{&self.props.accounts[idx].IBAN}</dd>
                                        <dt><span class="has-text-weight-bold">{"BIC:"}</span></dt><dd>{&self.props.accounts[idx].BIC}</dd>
                                        <dt><span class="has-text-weight-bold">{"Currency:"}</span></dt><dd>{&self.props.accounts[idx].currency}</dd>
                                    </dl>
                                    </ybc::Content>
                                </ybc::CardContent>
                            </ybc::Card>
                        }
                    } else {
                        html!{}
                    }
                }
            </ybc::Container>
        }
    }
}
