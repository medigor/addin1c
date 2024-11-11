use std::ffi::c_int;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Tm {
    pub sec: c_int,   // seconds after the minute - [0, 60] including leap second
    pub min: c_int,   // minutes after the hour - [0, 59]
    pub hour: c_int,  // hours since midnight - [0, 23]
    pub mday: c_int,  // day of the month - [1, 31]
    pub mon: c_int,   // months since January - [0, 11]
    pub year: c_int,  // years since 1900
    pub wday: c_int,  // days since Sunday - [0, 6]
    pub yday: c_int,  // days since January 1 - [0, 365]
    pub isdst: c_int, // daylight savings time flag

    #[cfg(target_family = "unix")]
    pub gmtoff: std::ffi::c_long, // seconds east of UTC
    #[cfg(target_family = "unix")]
    pub zone: std::ffi::c_char, // timezone abbreviation
}

#[cfg(feature = "chrono")]
mod chrono {
    use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, Timelike, Utc};

    use super::Tm;

    impl<T: Timelike + Datelike> From<T> for Tm {
        fn from(value: T) -> Self {
            Tm {
                sec: value.second() as _,
                min: value.minute() as _,
                hour: value.hour() as _,
                mday: value.day() as _,
                mon: value.month0() as _,
                year: value.year() - 1900,
                wday: value.weekday() as _,
                ..Tm::default()
            }
        }
    }

    impl From<Tm> for DateTime<Utc> {
        fn from(val: Tm) -> Self {
            Into::<NaiveDateTime>::into(val).and_utc()
        }
    }

    impl From<Tm> for NaiveDateTime {
        fn from(val: Tm) -> Self {
            NaiveDate::from_ymd_opt(1900 + val.year, 1 + val.mon as u32, val.mday as _)
                .expect("Incorrect date")
                .and_hms_opt(val.hour as _, val.min as _, val.sec as _)
                .expect("Incorrect date")
        }
    }

    #[cfg(test)]
    mod tests {

        use chrono::NaiveDate;

        use super::Tm;

        #[test]
        fn test() {
            let naive_datetime = NaiveDate::from_ymd_opt(2024, 1, 1)
                .unwrap()
                .and_hms_opt(1, 2, 3)
                .unwrap();

            let tm: Tm = naive_datetime.into();
            assert_eq!(naive_datetime, tm.into());

            let utc = naive_datetime.and_utc();
            let tm: Tm = utc.into();
            assert_eq!(utc, tm.into());
        }
    }
}
