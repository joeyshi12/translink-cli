# translink-next-bus

Tool to fetch Translink next bus times.

This is a CLI for https://www.translink.ca/schedules-and-maps/.

## Getting started

1. Clone the repository
    ```sh
    git clone https://github.com/joeyshi12/translink-next-bus
    ```
2. Install the crate
    ```sh
    cd translink-next-bus
    cargo install --path .
    ```
3. Run the command
    ```sh
    tlnb --stop-id <stop_id> --route-id <route_id>
    ```
