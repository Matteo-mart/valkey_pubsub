# Redis Pub/Sub Stream Subscriber

A lightweight, asynchronous Rust application that demonstrates how to subscribe to a Redis channel and process messages as a stream using `tokio` and `redis-rs`.

## Features

- **Asynchronous Execution**: Built with `tokio` for non-blocking I/O.
- **Stream-based Processing**: Utilizes `futures_util::StreamExt` to handle messages as an asynchronous stream.
- **Graceful Shutdown**: Includes a "quit" command listener to stop the subscriber safely.
- **Error Handling**: Implements `redis::RedisResult` for robust error management.

## Prerequisites

Before running this project, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Redis Server](https://redis.io/docs/getting-started/) running locally on the default port (`6379`).

## Installation

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/Matteo-mart/pubsub.git
    cd pubsub
    ```

2.  **Add dependencies to your `Cargo.toml`:**
    ```toml
    [dependencies]
    redis = { version = "0.24", features = ["tokio-comp"] }
    tokio = { version = "1.0", features = ["full"] }
    futures-util = "0.3"
    ```

## Usage

1.  **Start your Redis server:**
    ```bash
    redis-server
    ```

2.  **Run the Rust subscriber:**
    ```bash
    cargo run
    ```

3.  **Send messages via `redis-cli`:**
    Open a new terminal and use the following command to publish messages:
    ```bash
    redis-cli PUBLISH local_channel "Hello world!"
    ```

4.  **Stop the program:**
    Send the "quit" message to terminate the subscriber:
    ```bash
    redis-cli PUBLISH local_channel "quit"
    ```

## Code Overview

The application follows these steps:
- Establishes a connection to `redis://127.0.0.1:6379`.
- Subscribes to the `local_channel` channel.
- Converts the PubSub listener into a `Stream`.
- Iterates over the stream, printing the payload of each message.
- If a message contains the string `"quit"`, the loop breaks and the program exits.

## License

This project is open-source and available under the MIT License.
