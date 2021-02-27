use crate::create_travel_dialog::CreateTravelDialog;
use yew::prelude::*;

pub struct TravelBoard {
    _link: ComponentLink<Self>,
}

impl Component for TravelBoard {
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
            <div class=("travel-board-container")>
                <div class=("travel-board-header")>
                    <CreateTravelDialog />
                </div>
            </div>
        }
    }
}
