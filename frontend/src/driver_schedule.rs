use yew::prelude::*;

use crate::log;

pub struct DriverSchedule {
    link: ComponentLink<Self>,
}

impl Component for DriverSchedule {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=("driver-schedule")>
                <div class=("bp3-callout travel-container travel-hour-1 travel-precision-3")>
                    <div class=("depart")>{"Belmont"}</div>
                    <div class=("arrivee")>{"Lyon"}</div>
                </div>
                <div class=("bp3-callout travel-container travel-hour-1 travel-precision-3")>
                    <div class=("depart")>{"Lyon"}</div>
                    <div class=("arrivee")>{"Belmont"}</div>
                </div>
                <div class=("bp3-callout travel-container travel-hour-0 travel-precision-3")>
                    <div class=("depart")>{"Belmont"}</div>
                    <div class=("arrivee")>{"Villefranche sur saone"}</div>
                </div>
                <div class=("bp3-callout travel-container travel-hour-5 travel-precision-1")>
                    <div class=("depart")>{"Villefranche sur saone"}</div>
                    <div class=("arrivee")>{"Paris"}</div>
                </div>
                <div class=("bp3-callout travel-container travel-hour-6 travel-precision-3")>
                    <div class=("depart")>{"Paris"}</div>
                    <div class=("arrivee")>{"Bordeaux"}</div>
                </div>
            </div>
        }
    }
}