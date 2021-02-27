use crate::driver_selector::DriverSelector;
use crate::CreateDriverDialog;
use yew::prelude::*;

pub struct DriverBoard {
    _link: ComponentLink<Self>,
}

impl Component for DriverBoard {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { _link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=("driver-board-container")>
                <div class=("driver-board-header")>
                    <DriverSelector />
                    <CreateDriverDialog />
                </div>
            </div>
        }
    }
}
