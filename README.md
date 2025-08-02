# translink-next-bus

Tool to fetch Translink next bus times.

This is a CLI for https://www.translink.ca/schedules-and-maps/.

## Getting started

1. Make sure `${HOME}/.cargo/bin` is in your `${PATH}`
2. Install the crate with `cargo`
    ```sh
    cargo install --git https://github.com/joeyshi12/translink-next-bus
    ```
3. Run the command
    ```sh
    tlnb -s <stop_id> -r <route_id>
    ```
