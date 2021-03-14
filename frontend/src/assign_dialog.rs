use crate::driver_selector::{Driver, DriverSelector};
use yew::prelude::*;

use crate::log;

pub enum Msg {
    Close,
    Submit,
    InputDriver(Driver),
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub is_visible: bool,
    #[prop_or_default]
    pub selected_driver: Driver,
}

pub struct AssignDialog {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for AssignDialog {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Close => self.props.is_visible = false,
            Msg::Submit => {
                log!("{}", self.props.selected_driver);
                self.props.is_visible = false
            },
            Msg::InputDriver(e) => {
                log!("{:?}", e);
                self.props.selected_driver = e
            },
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
                                    <h4 class=("bp3-heading")>{"Assigner un voyage"}</h4>
                                </div>
                                <div class=("bp3-dialog-body")>
                                    <div class=("bp3-form-group")>
                                        <label class=("bp3-label") for=("from-input")>{ "Chauffeur" }</label>
                                        <div class=("bp3-form-content")>
                                            <DriverSelector selected_driver=self.props.selected_driver.clone() on_change=self.link.callback(|e: Driver| Msg::InputDriver(e)) />
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
            html! { <div></div> }
        }
    }
}
