use chrono::{DateTime, Datelike, Duration, FixedOffset, NaiveDateTime, Weekday};
use std::fmt::{Display, Formatter, Result};
use yew::prelude::*;

fn date_now() -> DateTime<FixedOffset> {
    let date: js_sys::Date = js_sys::Date::new_0();
    let millisecs_since_unix_epoch: u64 = date.get_time() as u64;
    let secs = millisecs_since_unix_epoch / 1000;
    let nanos = 1_000_000 * (millisecs_since_unix_epoch % 1000);
    let naive = NaiveDateTime::from_timestamp(secs as i64, nanos as u32);
    DateTime::<FixedOffset>::from_utc(naive, FixedOffset::east(1 * 3600))
}

#[derive(Clone, Debug)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Display for Month {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Month::January => write!(f, "Janvier"),
            Month::February => write!(f, "Fevrier"),
            Month::March => write!(f, "Mars"),
            Month::April => write!(f, "Avril"),
            Month::May => write!(f, "Mai"),
            Month::June => write!(f, "Juin"),
            Month::July => write!(f, "Juillet"),
            Month::August => write!(f, "Aout"),
            Month::September => write!(f, "Septembre"),
            Month::October => write!(f, "Octobre"),
            Month::November => write!(f, "Novembre"),
            Month::December => write!(f, "Decembre"),
        }
    }
}

impl From<u32> for Month {
    fn from(month: u32) -> Self {
        match month {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => panic!("not a month"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DatetimeWrapper {
    pub datetime: DateTime<FixedOffset>,
}

impl Default for DatetimeWrapper {
    fn default() -> Self {
        Self {
            datetime: date_now(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Date {
    pub day: u32,
    pub week_day: Weekday,
    pub month: Month,
    pub year: i32,
}

impl Default for Date {
    fn default() -> Self {
        let date_time = date_now();
        Self {
            day: date_time.day(),
            week_day: date_time.weekday(),
            month: date_time.month().into(),
            year: date_time.year(),
        }
    }
}

impl From<DateTime<FixedOffset>> for Date {
    fn from(date_time: DateTime<FixedOffset>) -> Self {
        Self {
            day: date_time.day(),
            week_day: date_time.weekday(),
            month: date_time.month().into(),
            year: date_time.year(),
        }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{} {} {} {}",
            self.week_day, self.day, self.month, self.year
        )
    }
}

pub enum Msg {
    Previous,
    Next,
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    #[prop_or_default]
    pub current_day: DatetimeWrapper,
    #[prop_or_default]
    pub date: Date,
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
            Msg::Previous => {
                self.props.current_day = DatetimeWrapper {
                    datetime: self.props.current_day.datetime - Duration::days(1),
                }
            }
            Msg::Next => {
                self.props.current_day = DatetimeWrapper {
                    datetime: self.props.current_day.datetime + Duration::days(1),
                }
            }
        }
        self.props.date = self.props.current_day.datetime.into();
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button class=("bp3-button") onclick=self.link.callback(|_| Msg::Previous)> { "Precedent" }</button>
                <div class=("bp3-text")>{ self.props.date.clone() }</div>
                <button class=("bp3-button") onclick=self.link.callback(|_| Msg::Next)> { "Suivant" }</button>
            </div>
        }
    }
}
