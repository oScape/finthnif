use crate::driver_schedule::DriverSchedule;
use crate::driver_selector::{Driver, DriverSelector};
use crate::timeline::Timeline;
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
                self.props.selected_driver = e;
                log!("{:?}", self.props.selected_driver);
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
                    <Timeline />
                    <DriverSchedule />
                </div>
            </div>
        }
    }
}
