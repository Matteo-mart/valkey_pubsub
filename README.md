Valkey GLIDE Go Demo

This project demonstrates how to perform basic database operations using the Valkey GLIDE Go driver. It includes connection setup, error handling, and common CRUD commands like GET, SET, and MSET.
 Getting Started
Prerequisites

    Go installed on your system.

    Valkey Server installed and running.

Environment Variables

The application looks for the following variables to configure the connection; otherwise, it defaults to localhost:6379:

    VALKEY_HOST: The server hostname.

    VALKEY_PORT: The server port.

 How to Run

    Start the Valkey Server:
    Bash

    valkey-server

    (Tip: Use clear && valkey-server to start with a clean terminal)

    Run the Application:
    Bash

    go run .

 Key Files

    connection.go: Manages client initialization, environment variable parsing, and connection testing.

    operation.go: Contains reusable functions for Set, Get, MSet, MGet, and Del operations.

    main.go: The entry point that orchestrates the workflow.

 Features Included

    Automatic Defaults: Falls back to standard local settings if environment variables are missing.

    Safety Checks: Performs a "connection test" write operation upon startup to ensure the host is accessible.

    Bulk Operations: Demonstrates how to handle multiple keys efficiently using MSet and MGet.
