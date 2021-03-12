#![recursion_limit = "1024"]

use crate::assign_dialog::AssignDialog;
use crate::create_driver_dialog::CreateDriverDialog;
use crate::day_selector::DaySelector;
use crate::driver_board::DriverBoard;
use crate::hour_selector::HourSelector;
use crate::travel_board::TravelBoard;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod assign_dialog;
mod create_driver_dialog;
mod create_travel_dialog;
mod day_selector;
mod driver_board;
mod driver_selector;
mod hour_selector;
mod travel_board;

struct App {}

impl Component for App {
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
            <div class=("app-root bp3-dark")>
                <DaySelector />
                <div class=("board-container")>
                    <DriverBoard />
                    <TravelBoard />
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<App>();
}

#[macro_export]
macro_rules! log {
    ($s:expr $(,$args:expr)*) => {{
        yew::services::ConsoleService::log(format!($s $(,$args)*).as_str());
    }};
}
