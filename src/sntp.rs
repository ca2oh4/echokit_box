use esp_idf_svc::sntp::{EspSntp, SyncStatus::Completed};
use std::{
    thread::sleep,
    time::{Duration, SystemTime},
};

pub fn sync_time() {
    log::info!("SNTP sync time");
    show_now();
    let ntp_client = EspSntp::new_default().unwrap();
    loop {
        let status = ntp_client.get_sync_status();
        log::debug!("sntp sync status {:?}", status);
        if status == Completed {
            show_now();
            break;
        }
        sleep(Duration::from_secs(1));
    }
    log::info!("SNTP synchronized!");
}

fn show_now() {
    let now = SystemTime::now();
    log::info!("now time: {:?}", now);
}
