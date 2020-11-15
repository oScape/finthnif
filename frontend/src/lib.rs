use crate::create_driver::CreateDriver;
use crate::day_selector::DaySelector;
use crate::driver_selector::DriverSelector;
use crate::hour_selector::HourSelector;
use chrono::DateTime;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod create_driver;
mod day_selector;
mod driver_selector;
mod hour_selector;

struct Model {}

impl Component for Model {
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
        let current_day = DateTime::parse_from_rfc2822("Wed, 18 Feb 2015 23:16:09 GMT").unwrap();

        html! {
            <div>
                <DaySelector current_day=current_day />
                <DriverSelector />
                <HourSelector />
                <CreateDriver firstname="Benoit".to_string() lastname="Chassignol".to_string() />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}

#[macro_export]
macro_rules! log {
    ($s:expr $(,$args:expr)*) => {{
        yew::services::ConsoleService::log(format!($s $(,$args)*).as_str());
    }};
}
