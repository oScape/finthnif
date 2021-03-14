use std::fmt::{Display, Formatter, Result};
use yew::prelude::*;
use yew_components::Select;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub selected_driver: Driver,
    pub on_change: Callback<Driver>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Driver {
    firstname: String,
}

impl Default for Driver {
    fn default() -> Self {
        Self { firstname: String::default() }
    }
}

impl Driver {
    pub fn new(firstname: String) -> Self {
        Self { firstname }
    }
}

impl Display for Driver {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.firstname)
    }
}

pub struct DriverSelector {
    props: Props,
    _link: ComponentLink<Self>,
}

impl Component for DriverSelector {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props, _link }
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
            <Select<Driver> class=("bp3-button") options=drivers on_change=self.props.on_change.clone() placeholder=("Selectionner un chauffeur")/>
        }
    }
}
