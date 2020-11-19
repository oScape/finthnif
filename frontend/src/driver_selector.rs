use std::fmt::{Display, Formatter, Result};
use yew::prelude::*;
use yew_components::Select;

#[derive(PartialEq, Clone)]
pub struct Driver {
    firstname: String,
}

impl Driver {
    fn new(firstname: String) -> Self {
        Self { firstname }
    }
}

impl Display for Driver {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.firstname)
    }
}

pub struct DriverSelector {
    link: ComponentLink<Self>,
}

impl Component for DriverSelector {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let drivers = vec![
            Driver::new("Franck".to_string()),
            Driver::new("Jean-Philippe".to_string()),
        ];

        html! {
            <Select<Driver> class=("bp3-button") options=drivers on_change=self.link.callback(|_| ()) placeholder=("Selectionner un chauffeur")/>
        }
    }
}
