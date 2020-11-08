use chrono::{DateTime, FixedOffset};
use yew::prelude::*;

pub enum Msg {
    Previous,
    Next,
    Current,
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub current_day: DateTime<FixedOffset>,
}

pub struct DaySelector {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for DaySelector {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <p>{ self.props.current_day }</p>
            </div>
        }
    }
}
