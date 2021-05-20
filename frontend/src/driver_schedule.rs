use yew::prelude::*;

pub struct DriverSchedule {}

impl Component for DriverSchedule {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=("driver-schedule")>
                <div class=("bp3-callout travel-container travel-hour-1 travel-precision-3")>
                    <div>{"Belmont"}</div>
                    <div>{"Lyon"}</div>
                </div>
                <div class=("bp3-callout travel-container travel-hour-1 travel-precision-3")>
                    <div>{"Lyon"}</div>
                    <div>{"Belmont"}</div>
                </div>
                <div class=("bp3-callout travel-container travel-hour-0 travel-precision-3")>
                    <div>{"Belmont"}</div>
                    <div>{"Villefranche sur saone"}</div>
                </div>
                <div class=("bp3-callout travel-container travel-hour-5 travel-precision-1")>
                    <div>{"Villefranche sur saone"}</div>
                    <div>{"Paris"}</div>
                </div>
                <div class=("bp3-callout travel-container travel-hour-6 travel-precision-3")>
                    <div>{"Paris"}</div>
                    <div>{"Bordeaux"}</div>
                </div>
            </div>
        }
    }
}
