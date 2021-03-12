use crate::assign_dialog::AssignDialog;
use crate::create_travel_dialog::CreateTravelDialog;
use yew::prelude::*;

pub enum Msg {
    Assign,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    assign_dialog_opened: bool,
}

pub struct TravelBoard {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for TravelBoard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Assign => self.props.assign_dialog_opened = true,
        }
        true
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
                <div class=("travel-board-body")>
                    <table class=("travel-board-table bp3-html-table bp3-html-table-striped bp3-interactive")>
                        <thead>
                            <tr>
                                <th>{"Depart"}</th>
                                <th>{"Arrivee"}</th>
                                <th>{"Assigner"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>{"Belmont de la loire"}</td>
                                <td>{"Lyon"}</td>
                                <td><button class=("bp3-button") onclick=self.link.callback(|_| Msg::Assign)>{"Assigner"}</button></td>
                            </tr>
                            <tr>
                                <td>{"Villefranche sur Saone"}</td>
                                <td>{"Belmont de la loire"}</td>
                                <td><button class=("bp3-button") onclick=self.link.callback(|_| Msg::Assign)>{"Assigner"}</button></td>
                            </tr>
                            <tr>
                                <td>{"Belmont de la loire"}</td>
                                <td>{"Bourg en Bresse"}</td>
                                <td><button class=("bp3-button") onclick=self.link.callback(|_| Msg::Assign)>{"Assigner"}</button></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
                <AssignDialog is_visible=self.props.assign_dialog_opened />
            </div>
        }
    }
}
