use ucelofka_data::customer::Customer;
use yew::prelude::*;

pub enum Messages {
    SelectCustomer(usize),
}

#[derive(Default, Properties, Clone, PartialEq)]
pub struct CustomersProps {
    #[prop_or_default]
    pub customers: Vec<Customer>,
    #[prop_or_default]
    pub selected_idx: Option<usize>,
}

pub struct Customers {
    props: CustomersProps,
    link: ComponentLink<Self>,
}

impl Component for Customers {
    type Message = Messages;
    type Properties = CustomersProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::SelectCustomer(idx) => {
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
                <ybc::Panel heading=html!{{"Customers"}}>
                {
                    for self.props.customers.iter().enumerate().map(|(idx, customer)| {
                        html! {
                            <ybc::PanelBlock
                                tag="a"
                                onclick=self.link.callback(move |_| Messages::SelectCustomer(idx))
                                active=Some(idx) == self.props.selected_idx
                            >
                                <span class="panel-icon"><i class="bi bi-caret-down-fill" aria-hidden="true"></i></span>
                                {&customer.id}
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
                                    <p class="card-header-title">{&self.props.customers[idx].id}</p>
                                </ybc::CardHeader>
                                <ybc::CardContent>
                                    <ybc::Content>
                                    <dl>
                                        <dt><span class="has-text-weight-bold">{"Name:"}</span></dt><dd>{&self.props.customers[idx].name}</dd>
                                        <dt><span class="has-text-weight-bold">{"Address:"}</span></dt>
                                        {
                                            for self.props.customers[idx].address.iter().map(|address_part| html!{
                                                <dd>{address_part}</dd>
                                            })

                                        }
                                        <dt><span class="has-text-weight-bold">{"Email:"}</span></dt>
                                        {
                                            for self.props.customers[idx].email.iter().map(|email| html!{
                                                <dd><a href={format!("mailto:{}", &email)}>{email}</a></dd>
                                            })

                                        }
                                        <dt><span class="has-text-weight-bold">{"Identification:"}</span></dt><dd>{&self.props.customers[idx].identification}</dd>
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
