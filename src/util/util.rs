use chrono::{Local, NaiveDateTime, Duration};

pub fn now() -> NaiveDateTime {
  Local::now().naive_local()
}

pub fn default_expire() -> NaiveDateTime {
  Local::now().naive_local() + Duration::days(30)
}