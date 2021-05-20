use yew::prelude::*;

pub struct Timeline {}

impl Component for Timeline {
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
        }
    }
}
