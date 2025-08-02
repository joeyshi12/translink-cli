use clap::Parser;
mod translink;

#[derive(Parser, Debug)]
#[command(version, about, long_about = Some("Tool to fetch Translink next bus times"))]
struct Args {
    #[arg(short, long)]
    stop_id: String,
    #[arg(short, long)]
    route_id: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let schedules_result = translink::fetch_bus_schedules(&args.stop_id, &args.route_id).await;
    match schedules_result {
        Ok(schedules) => translink::print_schedules(&schedules),
        Err(e) => eprintln!("Failed to fetch bus times - {}", e),
    };
}
