use ucelofka_data::entry::Entry;
use yew::prelude::*;

pub enum Messages {
    SelectEntry(usize),
}

#[derive(Default, Properties, Clone, PartialEq)]
pub struct EntriesProps {
    #[prop_or_default]
    pub entries: Vec<Entry>,
    #[prop_or_default]
    pub selected_idx: Option<usize>,
}

pub struct Entries {
    props: EntriesProps,
    link: ComponentLink<Self>,
}

impl Component for Entries {
    type Message = Messages;
    type Properties = EntriesProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::SelectEntry(idx) => {
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
                <ybc::Panel heading=html!{{"Entries"}}>
                {
                    for self.props.entries.iter().enumerate().map(|(idx, entry)| {
                        html! {
                            <ybc::PanelBlock
                                tag="a"
                                onclick=self.link.callback(move |_| Messages::SelectEntry(idx))
                                active=Some(idx) == self.props.selected_idx
                            >
                                <span class="panel-icon"><i class="bi bi-caret-down-fill" aria-hidden="true"></i></span>
                                {&entry.id}
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
                                    <p class="card-header-title">{&self.props.entries[idx].id}</p>
                                </ybc::CardHeader>
                                <ybc::CardContent>
                                    <ybc::Content>
                                    <dl>
                                        <dt><span class="has-text-weight-bold">{"Name:"}</span></dt><dd>{&self.props.entries[idx].name}</dd>
                                        <dt><span class="has-text-weight-bold">{"Price:"}</span></dt><dd>{&self.props.entries[idx].price}</dd>
                                        <dt><span class="has-text-weight-bold">{"Currency:"}</span></dt><dd>{&self.props.entries[idx].currency}</dd>
                                        <dt><span class="has-text-weight-bold">{"Details:"}</span></dt>
                                        {
                                            for self.props.entries[idx].details.iter().map(|details_part| html!{
                                                <dd>{details_part}</dd>
                                            })

                                        }
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
