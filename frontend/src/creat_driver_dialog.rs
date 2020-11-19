use yew::prelude::*;

pub enum Msg {
    Open,
    Close,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    is_visible: bool,
}

pub struct CreateDriverDialog {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for CreateDriverDialog {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Open => self.props.is_visible = true,
            Msg::Close => self.props.is_visible = false,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        if self.props.is_visible {
            html! {
                <div class=("bp3-portal")>
                    <div class=("bp3-overlay bp3-overlay-open bp3-overlay-scroll-container")>
                        <div class=("bp3-overlay-backdrop bp3-overlay-enter-done")></div>
                        <div class=("bp3-dialog-container bp3-overlay-content bp3-overlay-enter-done")>
                            <div class=("bp3-dialog bp3-dark")>
                                <div class=("bp3-dialog-header")>
                                    <h4 class=("bp3-heading")>{"Dialog header"}</h4>
                                </div>
                                <div class=("bp3-dialog-body")>
                                    {"Yo"}
                                </div>
                                <div class=("bp3-dialog-footer")>
                                    <button class=("bp3-button") onclick=self.link.callback(|_| Msg::Close)>{"Close"}</button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            }
        } else {
            html! {
                <button class=("bp3-button") onclick=self.link.callback(|_| Msg::Open)> { "Create driver" }</button>
            }
        }
    }
}
