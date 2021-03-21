use crate::driver_selector::{Driver, DriverSelector};
use crate::CreateDriverDialog;
use yew::prelude::*;

use crate::log;

pub enum Msg {
    InputDriver(Driver),
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub selected_driver: Driver,
}

pub struct DriverBoard {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for DriverBoard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InputDriver(e) => {
                log!("{:?}", e);
                self.props.selected_driver = e
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=("driver-board-container")>
                <div class=("driver-board-header")>
                    <DriverSelector on_change=self.link.callback(|e: Driver| Msg::InputDriver(e)) />
                    <CreateDriverDialog />
                </div>
                <div class=("driver-board-body")>
                    <div class=("timeline")>
                        <div class=("hour-container")>
                            <div class=("hour")>{"5:00"}</div>
                            <div class=("precision")>{":15"}</div>
                            <div class=("precision")>{":30"}</div>
                            <div class=("precision")>{":45"}</div>
                        </div>
                        <div class=("hour-container")>
                            <div class=("hour")>{"6:00"}</div>
                            <div class=("precision")>{":15"}</div>
                            <div class=("precision")>{":30"}</div>
                            <div class=("precision")>{":45"}</div>
                        </div>
                        <div class=("hour-container")>
                            <div class=("hour")>{"7:00"}</div>
                            <div class=("precision")>{":15"}</div>
                            <div class=("precision")>{":30"}</div>
                            <div class=("precision")>{":45"}</div>
                        </div>
                        <div class=("hour-container")>
                            <div class=("hour")>{"8:00"}</div>
                            <div class=("precision")>{":15"}</div>
                            <div class=("precision")>{":30"}</div>
                            <div class=("precision")>{":45"}</div>
                        </div>
                        <div class=("hour-container")>
                            <div class=("hour")>{"9:00"}</div>
                            <div class=("precision")>{":15"}</div>
                            <div class=("precision")>{":30"}</div>
                            <div class=("precision")>{":45"}</div>
                        </div>
                        <div class=("hour-container")>
                            <div class=("hour")>{"10:00"}</div>
                            <div class=("precision")>{":15"}</div>
                            <div class=("precision")>{":30"}</div>
                            <div class=("precision")>{":45"}</div>
                        </div>
                        <div class=("hour-container")>
                            <div class=("hour")>{"11:00"}</div>
                            <div class=("precision")>{":15"}</div>
                            <div class=("precision")>{":30"}</div>
                            <div class=("precision")>{":45"}</div>
                        </div>
                        <div class=("hour-container")>
                            <div class=("hour")>{"12:00"}</div>
                            <div class=("precision")>{":15"}</div>
                            <div class=("precision")>{":30"}</div>
                            <div class=("precision")>{":45"}</div>
                        </div>
                        <div class=("hour-container")>
                            <div class=("hour")>{"13:00"}</div>
                            <div class=("precision")>{":15"}</div>
                            <div class=("precision")>{":30"}</div>
                            <div class=("precision")>{":45"}</div>
                        </div>
                        <div class=("hour-container")>
                            <div class=("hour")>{"14:00"}</div>
                            <div class=("precision")>{":15"}</div>
                            <div class=("precision")>{":30"}</div>
                            <div class=("precision")>{":45"}</div>
                        </div>
                        <div class=("hour-container")>
                            <div class=("hour")>{"15:00"}</div>
                            <div class=("precision")>{":15"}</div>
                            <div class=("precision")>{":30"}</div>
                            <div class=("precision")>{":45"}</div>
                        </div>
                        <div class=("hour-container")>
                            <div class=("hour")>{"16:00"}</div>
                            <div class=("precision")>{":15"}</div>
                            <div class=("precision")>{":30"}</div>
                            <div class=("precision")>{":45"}</div>
                        </div>
                        <div class=("hour-container")>
                            <div class=("hour")>{"17:00"}</div>
                            <div class=("precision")>{":15"}</div>
                            <div class=("precision")>{":30"}</div>
                            <div class=("precision")>{":45"}</div>
                        </div>
                        <div class=("hour-container")>
                            <div class=("hour")>{"18:00"}</div>
                            <div class=("precision")>{":15"}</div>
                            <div class=("precision")>{":30"}</div>
                            <div class=("precision")>{":45"}</div>
                        </div>
                        <div class=("hour-container")>
                            <div class=("hour")>{"19:00"}</div>
                            <div class=("precision")>{":15"}</div>
                            <div class=("precision")>{":30"}</div>
                            <div class=("precision")>{":45"}</div>
                        </div>
                        <div class=("hour-container")>
                            <div class=("hour")>{"20:00"}</div>
                            <div class=("precision")>{":15"}</div>
                            <div class=("precision")>{":30"}</div>
                            <div class=("precision")>{":45"}</div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
