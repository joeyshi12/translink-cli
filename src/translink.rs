use std::fmt::Debug;
use chrono::{Local, NaiveTime, Timelike};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BusSchedule {
    id: String,
    sc: String,
    sn: String,
    r: Vec<BusRoute>,
}

#[derive(Serialize, Deserialize, Debug)]
struct BusRoute {
    ri: String,
    rs: String,
    hs: String,
    t: Vec<BusTime>
}

#[derive(Serialize, Deserialize, Debug)]
struct BusTime {
    ti: String,
    dt: String,
}

pub async fn fetch_bus_schedules(stop_id: &str, route_id: &str) -> Result<Vec<BusSchedule>, reqwest::Error> {
    let url = format!(
        "https://getaway.translink.ca/api/gtfs/stop/{}/route/{}/realtimeschedules?querySize=6",
        stop_id,
        route_id
    );
    let response = reqwest::get(url).await?;
    Ok(response.json::<Vec<BusSchedule>>().await?)
}

pub fn print_schedules(schedules: &Vec<BusSchedule>) {
    let now = Local::now();
    for schedule in schedules.iter() {
        println!("Stop #{} - {}", schedule.sc, schedule.sn);
        for route in schedule.r.iter() {
            println!("{} {}", route.rs, route.hs);
            let departing_times = route.t.iter()
                .map(|time: &BusTime| format_departing_time(&time, &now))
                .collect::<Vec<String>>()
                .join(", ");
            println!("Departing: {}", departing_times);
        }
        println!();
    }
}

fn format_departing_time(bus_time: &BusTime, now: &chrono::DateTime<Local>) -> String {
    let parsed_time = NaiveTime::parse_from_str(&bus_time.dt, "%H:%M")
        .expect("Failed to parse time");
    let mut hours_diff = (parsed_time.hour() as i32) - (now.hour() as i32);
    let mut minutes_diff = (parsed_time.minute() as i32) - (now.minute() as i32);
    if minutes_diff < 0 {
        minutes_diff = minutes_diff.rem_euclid(60);
        hours_diff -= 1;
    }
    hours_diff = hours_diff.rem_euclid(24);
    if hours_diff == 0 {
        format!("{}min", minutes_diff)
    } else {
        format!("{}hrs {}min", hours_diff, minutes_diff)
    }
}
