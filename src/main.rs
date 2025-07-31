use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct BusSchedule {
    id: String,
    sn: String,
    sd: String
    //r: BusRoute
}

//#[derive(Deserialize)]
//struct BusRoute {
//    ri: String,
//    rs: String,
//    rl: String,
//    dn: String,
//    hs: String,
//    t: Vec<BusTime>
//}
//
//#[derive(Deserialize)]
//struct BusTime {
//    ri: String,
//    rs: String,
//    rl: String,
//    dn: String,
//    hs: String,
//    t: Vec<BusTime>
//}

fn main() {
    println!("Hello, world!");
}

