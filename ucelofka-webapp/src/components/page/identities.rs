use ucelofka_data::identity::Identity;
use yew::prelude::*;

pub enum Messages {
    SelectIdentity(usize),
}

#[derive(Default, Properties, Clone, PartialEq)]
pub struct IdentitiesProps {
    #[prop_or_default]
    pub identities: Vec<Identity>,
    #[prop_or_default]
    pub selected_idx: Option<usize>,
}

pub struct Identities {
    props: IdentitiesProps,
    link: ComponentLink<Self>,
}

impl Component for Identities {
    type Message = Messages;
    type Properties = IdentitiesProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::SelectIdentity(idx) => {
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
                <ybc::Panel heading=html!{{"Identities"}}>
                {
                    for self.props.identities.iter().enumerate().map(|(idx, identity)| {
                        html! {
                            <>
                                <ybc::PanelBlock
                                    tag="a"
                                    onclick=self.link.callback(move |_| Messages::SelectIdentity(idx))
                                    active=Some(idx) == self.props.selected_idx
                                >
                                    <span class="panel-icon"><i class="bi bi-caret-down-fill" aria-hidden="true"></i></span>
                                    {&identity.id}
                                </ybc::PanelBlock>
                            </>
                        }
                    })
                }
                </ybc::Panel>
                {
                    if let Some(idx) = self.props.selected_idx {
                        html!{
                            <ybc::Card>
                                <ybc::CardHeader>
                                    <p class="card-header-title">{&self.props.identities[idx].id}</p>
                                </ybc::CardHeader>
                                <ybc::CardContent>
                                    <ybc::Content>
                                    <dl>
                                        <dt><span class="has-text-weight-bold">{"Name:"}</span></dt><dd>{&self.props.identities[idx].name}</dd>
                                        <dt><span class="has-text-weight-bold">{"Address:"}</span></dt>
                                        {
                                            for self.props.identities[idx].address.iter().map(|address_part| html!{
                                                <dd>{address_part}</dd>
                                            })

                                        }
                                        <dt><span class="has-text-weight-bold">{"Phone:"}</span></dt>
                                        {
                                            for self.props.identities[idx].phone.iter().map(|phone| html!{
                                                <dd><a href={format!("tel:{}", &phone)}>{phone}</a></dd>
                                            })

                                        }
                                        <dt><span class="has-text-weight-bold">{"Email:"}</span></dt>
                                        {
                                            for self.props.identities[idx].email.iter().map(|email| html!{
                                                <dd><a href={format!("mailto:{}", &email)}>{email}</a></dd>
                                            })

                                        }
                                        <dt><span class="has-text-weight-bold">{"Identitfication:"}</span></dt>
                                        <dd>
                                            <dl>
                                                <dt><span class="has-text-weight-bold">{"Tax:"}</span></dt>
                                                <dd>{&self.props.identities[idx].identification.tax}</dd>
                                                <dt><span class="has-text-weight-bold">{"Registration:"}</span></dt>
                                                <dd>{&self.props.identities[idx].identification.registration}</dd>
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
