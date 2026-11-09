use amane::Service;
use std::{
    sync::OnceLock,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use time::{
    OffsetDateTime, UtcOffset, format_description::BorrowedFormatItem, macros::format_description,
};

const TIME_FMT: &[BorrowedFormatItem<'static>] =
    format_description!("[hour repr:12 padding:none]:[minute] [period]");
const DATE_FMT: &[BorrowedFormatItem<'static>] =
    format_description!("[month repr:short] [day padding:none]");

static OFFSET: OnceLock<UtcOffset> = OnceLock::new();

pub fn init_offset() {
    let _ = OFFSET.set(UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC));
}

fn now() -> OffsetDateTime {
    let offset = *OFFSET.get().unwrap_or(&UtcOffset::UTC);
    OffsetDateTime::now_utc().to_offset(offset)
}

#[derive(Default)]
pub struct Clock {
    pub time: String,
    pub date: String,
    day: u8,
}

impl Clock {
    fn refresh(&mut self) -> bool {
        let now = now();
        let mut changed = false;

        let time = now.format(TIME_FMT).unwrap();
        if self.time != time {
            self.time = time;
            changed = true;
        }

        if self.day != now.day() {
            self.day = now.day();
            self.date = now.format(DATE_FMT).unwrap();
            changed = true;
        }

        changed
    }
}

impl Service for Clock {
    fn new() -> Self {
        // populated immediately so the first render already has values
        let mut clock = Self::default();
        clock.refresh();
        clock
    }

    fn interval() -> Duration {
        let into_minute = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            % 60_000_000_000;

        Duration::from_nanos((60_000_000_000 - into_minute) as u64) + Duration::from_millis(20)
    }

    fn update(&mut self) -> bool {
        self.refresh()
    }
}
