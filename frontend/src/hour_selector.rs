use std::fmt::{Display, Formatter, Result};
use yew::prelude::*;
use yew_components::Select;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub on_change_hour: Callback<Hour>,
    pub on_change_precision: Callback<Precision>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Hour {
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
}

impl Display for Hour {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Hour::Five => write!(f, "{}", "5"),
            Hour::Six => write!(f, "{}", "6"),
            Hour::Seven => write!(f, "{}", "7"),
            Hour::Eight => write!(f, "{}", "8"),
            Hour::Nine => write!(f, "{}", "9"),
            Hour::Ten => write!(f, "{}", "10"),
            Hour::Eleven => write!(f, "{}", "11"),
            Hour::Twelve => write!(f, "{}", "12"),
            Hour::Thirteen => write!(f, "{}", "13"),
            Hour::Fourteen => write!(f, "{}", "14"),
            Hour::Fifteen => write!(f, "{}", "15"),
            Hour::Sixteen => write!(f, "{}", "16"),
            Hour::Seventeen => write!(f, "{}", "17"),
            Hour::Eighteen => write!(f, "{}", "18"),
            Hour::Nineteen => write!(f, "{}", "19"),
            Hour::Twenty => write!(f, "{}", "20"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Precision {
    Oclock,
    QuarterPast,
    Half,
    QuarterTo,
}

impl Display for Precision {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Precision::Oclock => write!(f, "{}", "00"),
            Precision::QuarterPast => write!(f, "{}", "15"),
            Precision::Half => write!(f, "{}", "30"),
            Precision::QuarterTo => write!(f, "{}", "45"),
        }
    }
}

impl Default for Hour {
    fn default() -> Self {
        Hour::Five
    }
}

impl Default for Precision {
    fn default() -> Self {
        Precision::Oclock
    }
}

pub struct HourSelector {
    props: Props,
    _link: ComponentLink<Self>,
}

impl Component for HourSelector {
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
        let hours = vec![
            Hour::Five,
            Hour::Six,
            Hour::Seven,
            Hour::Eight,
            Hour::Nine,
            Hour::Ten,
            Hour::Eleven,
            Hour::Twelve,
            Hour::Thirteen,
            Hour::Fourteen,
            Hour::Fifteen,
            Hour::Sixteen,
            Hour::Seventeen,
            Hour::Eighteen,
            Hour::Nineteen,
            Hour::Twenty,
        ];
        let precisions = vec![
            Precision::Oclock,
            Precision::QuarterPast,
            Precision::Half,
            Precision::QuarterTo,
        ];
        html! {
            <div>
                <Select<Hour> class=("bp3-button") options=hours on_change=self.props.on_change_hour.clone() placeholder=("Heure") />
                <Select<Precision> class=("bp3-button") options=precisions on_change=self.props.on_change_precision.clone() placeholder=("Precision") />
            </div>
        }
    }
}
