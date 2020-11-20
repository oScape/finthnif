use yew::prelude::*;

use crate::log;

pub enum Msg {
    Open,
    Close,
    Submit,
    InputLastname(InputData),
    InputFirstname(InputData),
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    is_visible: bool,
    #[prop_or_default]
    lastname: String,
    #[prop_or_default]
    firstname: String,
}

pub struct CreateDriverDialog {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for CreateDriverDialog {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Open => self.props.is_visible = true,
            Msg::Close => self.props.is_visible = false,
            Msg::Submit => {
                log!("{}, {}", self.props.lastname, self.props.firstname);
                self.props.is_visible = false
            }
            Msg::InputLastname(e) => self.props.lastname = e.value,
            Msg::InputFirstname(e) => self.props.firstname = e.value,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        if self.props.is_visible {
            html! {
                <div class=("bp3-portal")>
                    <div class=("bp3-overlay bp3-overlay-open bp3-overlay-scroll-container")>
                        <div class=("bp3-overlay-backdrop bp3-overlay-enter-done")></div>
                        <div class=("bp3-dialog-container bp3-overlay-content bp3-overlay-enter-done")>
                            <div class=("bp3-dialog bp3-dark")>
                                <div class=("bp3-dialog-header")>
                                    <h4 class=("bp3-heading")>{"Creer un chauffeur"}</h4>
                                </div>
                                <div class=("bp3-dialog-body")>
                                    <div class=("bp3-form-group")>
                                        <label class=("bp3-label") for=("lastname-input")>{ "Nom de famille" }</label>
                                        <div class=("bp3-form-content")>
                                            <div class=("bp3-input-group")>
                                                <input id=("lastname-input") class=("bp3-input") type=("text") oninput=self.link.callback(|e: InputData| Msg::InputLastname(e)) />
                                            </div>
                                        </div>
                                    </div>
                                    <div class=("bp3-form-group")>
                                        <label class=("bp3-label") for=("firstname-input")>{ "Prenom" }</label>
                                        <div class=("bp3-form-content")>
                                            <div class=("bp3-input-group")>
                                                <input id=("firstname-input") class=("bp3-input") type=("text") oninput=self.link.callback(|e: InputData| Msg::InputFirstname(e)) />
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <div class=("bp3-dialog-footer")>
                                    <div class=("bp3-dialog-footer-actions")>
                                        <span class=("bp3-popover-wrapper")>
                                            <span class=("bp3-popover-target")>
                                                <button class=("bp3-button") onclick=self.link.callback(|_| Msg::Close)>{"Close"}</button>
                                            </span>
                                        </span>
                                        <button class=("bp3-button bp3-intent-primary") onclick=self.link.callback(|_| Msg::Submit)>{"Valider"}</button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            }
        } else {
            html! {
                <button class=("bp3-button") onclick=self.link.callback(|_| Msg::Open)> { "Create driver" }</button>
            }
        }
    }
}
