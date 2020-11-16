use yew::prelude::*;

pub enum Msg {
    Create,
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub firstname: String,
    pub lastname: String,
}

pub struct CreateDriver {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for CreateDriver {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Create => crate::log!(
                "lastname: {}, firstname: {}",
                self.props.firstname,
                self.props.lastname
            ),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button class=("bp3-button") onclick=self.link.callback(|_| Msg::Create)> { "Create" }</button>
                <div class=("bp3-dialog-container")>
                    <div class=("bp3-dialog")>
                        <div class=("bp3-dialog-header")>
                            <h4 class=("bp3-heading")>{"Dialog header"}</h4>
                        </div>
                        <div class=("bp3-dialog-body")>
                            {"This dialog hasn't been wired up with any open or close interactions.
                            It's just an example of markup and styles."}
                        </div>
                        <div class=("bp3-dialog-footer")>
                            <button class=("bp3-button")>{"Secondary button"}</button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
