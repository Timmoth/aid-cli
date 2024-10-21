use time::format_description::well_known::Rfc3339;
use time::macros;
use time::OffsetDateTime;

pub fn unix_timestamp(milli: bool) {
    let now = OffsetDateTime::now_utc();

    if milli {
        let timestamp_milliseconds = now.unix_timestamp_nanos() / 1_000_000;
        println!("{}", timestamp_milliseconds);
    } else {
        let timestamp_seconds = now.unix_timestamp();
        println!("{}", timestamp_seconds);
    }
}

pub fn date_time(local: bool, rfc: bool) {
    let now: OffsetDateTime = if local {
        OffsetDateTime::now_local().expect("")
    } else {
        OffsetDateTime::now_utc()
    };
    
    if rfc {
        let formatted = now.format(&Rfc3339).unwrap();
        println!("{}", formatted);
    } else {
        let format = macros::format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
        let formatted = now.format(&format).unwrap();
        println!("{}", formatted);
    }
}