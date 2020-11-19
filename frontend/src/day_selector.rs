use chrono::{DateTime, Duration, FixedOffset};
use yew::prelude::*;

pub enum Msg {
    Previous,
    Next,
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

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Previous => self.props.current_day = self.props.current_day - Duration::days(1),
            Msg::Next => self.props.current_day = self.props.current_day + Duration::days(1),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button class=("bp3-button") onclick=self.link.callback(|_| Msg::Previous)> { "Precedent" }</button>
                <div class=("bp3-text")>{ self.props.current_day }</div>
                <button class=("bp3-button") onclick=self.link.callback(|_| Msg::Next)> { "Suivant" }</button>
            </div>
        }
    }
}
