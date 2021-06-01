use ucelofka_data::invoice::Invoice;
use yew::prelude::*;

pub enum Messages {
    SelectInvoice(usize),
}

#[derive(Default, Properties, Clone, PartialEq)]
pub struct InvoicesProps {
    #[prop_or_default]
    pub invoices: Vec<Invoice>,
    #[prop_or_default]
    pub selected_idx: Option<usize>,
}

pub struct Invoices {
    props: InvoicesProps,
    link: ComponentLink<Self>,
}

impl Invoices {
    fn current_invoice(&self) -> Option<&Invoice> {
        self.props.selected_idx.map(|idx| &self.props.invoices[idx])
    }
}

impl Component for Invoices {
    type Message = Messages;
    type Properties = InvoicesProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::SelectInvoice(idx) => {
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
                <ybc::Panel heading=html!{{"Invoices"}}>
                {
                    for self.props.invoices.iter().enumerate().map(|(idx, invoice)| {
                        html! {
                            <ybc::PanelBlock
                                tag="a"
                                onclick=self.link.callback(move |_| Messages::SelectInvoice(idx))
                                active=Some(idx) == self.props.selected_idx
                            >
                                <span class="panel-icon"><i class="bi bi-caret-down-fill" aria-hidden="true"></i></span>
                                {&invoice.id}
                            </ybc::PanelBlock>
                        }
                    })
                }
                </ybc::Panel>
                {
                    if let Some(invoice) = self.current_invoice() {
                        html!{
                            <ybc::Card>
                                <ybc::CardHeader>
                                    <p class="card-header-title">{&invoice.id}</p>
                                </ybc::CardHeader>
                                <ybc::CardContent>
                                    <ybc::Content>
                                    <dl>
                                        <dt><span class="has-text-weight-bold">{"Issue date:"}</span></dt><dd>{&invoice.issue_date}</dd>
                                        <dt><span class="has-text-weight-bold">{"Due date:"}</span></dt><dd>{&invoice.due_date}</dd>
                                        <dt><span class="has-text-weight-bold">{"Issuer:"}</span></dt>
                                        <dd>
                                            <dl>
                                                <dt><span class="has-text-weight-bold">{"Name:"}</span></dt><dd>{&invoice.issuer.name}</dd>
                                                <dt><span class="has-text-weight-bold">{"Address:"}</span></dt>
                                                {
                                                    for invoice.issuer.address.iter().map(|address_part| html!{
                                                        <dd>{address_part}</dd>
                                                    })
                                                }
                                                <dt><span class="has-text-weight-bold">{"Phone:"}</span></dt>
                                                {
                                                    for invoice.issuer.phone.iter().map(|phone| html!{
                                                        <dd><a href={format!("tel:{}", &phone)}>{phone}</a></dd>
                                                    })
                                                }
                                                <dt><span class="has-text-weight-bold">{"Email:"}</span></dt>
                                                {
                                                    for invoice.issuer.email.iter().map(|email| html!{
                                                        <dd><a href={format!("mailto:{}", &email)}>{email}</a></dd>
                                                    })
                                                }
                                                <dt><span class="has-text-weight-bold">{"WWW:"}</span></dt>
                                                {
                                                    for invoice.issuer.www.iter().map(|www| html!{
                                                        <dd><a href={format!("{}", www)}>{www}</a></dd>
                                                    })
                                                }
                                                <dt><span class="has-text-weight-bold">{"Identification:"}</span></dt>
                                                <dd>
                                                    <dl>
                                                    {
                                                        for invoice.issuer.identifications.iter().map(|identification| html!{
                                                            <>
                                                                <dt><span class="has-text-weight-bold">{format!("{}:", &identification.name)}</span></dt>
                                                                <dd>{&identification.value}</dd>
                                                            </>
                                                        })
                                                    }
                                                    </dl>
                                                </dd>
                                            </dl>
                                        </dd>

                                        <dt><span class="has-text-weight-bold">{"Customer:"}</span></dt>
                                        <dd>
                                            <dl>
                                                <dt><span class="has-text-weight-bold">{"Name:"}</span></dt><dd>{&invoice.customer.name}</dd>
                                                <dt><span class="has-text-weight-bold">{"Address:"}</span></dt>
                                                {
                                                    for invoice.customer.address.iter().map(|address_part| html!{
                                                        <dd>{address_part}</dd>
                                                    })
                                                }
                                                <dt><span class="has-text-weight-bold">{"Email:"}</span></dt>
                                                {
                                                    for invoice.customer.email.iter().map(|email| html!{
                                                        <dd><a href={format!("mailto:{}", &email)}>{email}</a></dd>
                                                    })
                                                }
                                                <dt><span class="has-text-weight-bold">{"Identification:"}</span></dt>
                                                <dd>
                                                    <dl>
                                                    {
                                                        for invoice.customer.identifications.iter().map(|identification| html!{
                                                            <>
                                                                <dt><span class="has-text-weight-bold">{format!("{}:", &identification.name)}</span></dt>
                                                                <dd>{&identification.value}</dd>
                                                            </>
                                                        })
                                                    }
                                                    </dl>
                                                </dd>
                                            </dl>
                                        </dd>

                                        <dt><span class="has-text-weight-bold">{"Entries:"}</span></dt>
                                        {
                                            for invoice.entries.iter().map(|entry| html!{
                                                <ul>
                                                        <li>
                                                            <dt><span class="has-text-weight-bold">{"Name:"}</span></dt><dd>{&entry.name}</dd>
                                                            <dt><span class="has-text-weight-bold">{"Price:"}</span></dt><dd>{&entry.price}</dd>
                                                            <dt><span class="has-text-weight-bold">{"Currency:"}</span></dt><dd>{&entry.currency}</dd>
                                                            <dt><span class="has-text-weight-bold">{"Details"}</span></dt>
                                                            {
                                                                for entry.details.iter().map(|detail_part| html!{
                                                                    <dd>{detail_part}</dd>
                                                                })
                                                            }
                                                        </li>
                                                </ul>
                                            })
                                        }

                                        <dt><span class="has-text-weight-bold">{"Billing:"}</span></dt>
                                        <dd>
                                            <dl>
                                                <dt><span class="has-text-weight-bold">{"Account name:"}</span></dt><dd>{&invoice.billing.account_name}</dd>
                                                <dt><span class="has-text-weight-bold">{"Account number:"}</span></dt><dd>{&invoice.billing.account_number}</dd>
                                                <dt><span class="has-text-weight-bold">{"BIC:"}</span></dt><dd>{&invoice.billing.BIC}</dd>
                                                <dt><span class="has-text-weight-bold">{"IBAN:"}</span></dt><dd>{&invoice.billing.IBAN}</dd>
                                                <dt><span class="has-text-weight-bold">{"Total:"}</span></dt><dd>{&invoice.billing.total}</dd>
                                                <dt><span class="has-text-weight-bold">{"Currency:"}</span></dt><dd>{&invoice.billing.currency}</dd>
                                                <dt><span class="has-text-weight-bold">{"Variable symbol:"}</span></dt><dd>{&invoice.billing.variable_symbol}</dd>
                                            </dl>
                                        </dd>
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
