# API Key Rotator

![Rust Version](https://img.shields.io/badge/rust-2021-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

A high-performance, concurrent backend service for intelligently managing and rotating third-party API keys. Built with Rust, Axum, and Tokio.

## Overview

This project provides a robust solution for applications that rely on multiple third-party API keys (e.g., for services like OpenAI, Google Gemini, etc.). Instead of using a single key and risking rate-limit exhaustion, this service rotates keys based on predefined strategies, ensuring high availability and balanced usage.

It operates without an external database, using a simple `keys.json` file for persistence, making it lightweight and easy to deploy.

## Features

-   **Intelligent Key Rotation**: Delivers the next available key based on different strategies.
-   **Multiple Rotation Modes**:
    -   `auto`: Returns the key that has been unused for the longest time.
    -   `random`: Returns a random, available key.
-   **Rate Limiting**: Enforces both per-key and global rate limits to prevent exhaustion.
    -   Requests Per Minute (RPM)
    -   Requests Per Day (RPD)
    -   Total Processed per Minute (TPM)
-   **Concurrent & Performant**: Built on `tokio` and uses `DashMap` for safe, lock-free concurrent access.
-   **File-based Persistence**: Key state is automatically saved to `keys.json` and loaded on restart.
-   **Full Management API**: Endpoints to add, delete, deactivate, and reactivate keys.
-   **Automated Background Tasks**: Handles periodic saving, usage counter resets, and key cooldowns.

## Technology Stack

-   **Backend**: Rust 2021 Edition
-   **Web Framework**: Axum
-   **Asynchronous Runtime**: Tokio
-   **Concurrent State Management**: DashMap
-   **Serialization/Deserialization**: Serde & Serde JSON

## Getting Started

### Prerequisites

-   [Rust and Cargo](https://www.rust-lang.org/tools/install) (latest stable version recommended)

### Build and Run

1.  **Clone the repository:**
    ```sh
    git clone <repository-url>
    cd api-key-rotator
    ```

2.  **Create your key storage file:**
    Create a `keys.json` file in the root of the project. This file will store your API keys. You can start with an empty list:
    ```json
    []
    ```
    See the [Configuration](#configuration) section for the full format.

3.  **Build the project:**
    ```sh
    cargo build --release
    ```

4.  **Run the server:**
    ```sh
    cargo run --release
    ```
    The server will start and listen on `0.0.0.0:8080`.

## Configuration

### Key Storage (`keys.json`)

The server stores all key data in this file. Each key object has the following structure:

```json
[
  {
    "key": "your-secret-api-key-string",
    "status": "Active",
    "usage": {
      "requests_this_minute": 0,
      "requests_this_day": 0
    },
    "last_used": "2023-11-04T12:00:00Z",
    "created_at": "2023-11-04T12:00:00Z",
    "deactivated_at": null
  }
]
```
-   `status`: Can be `"Active"` or `"Inactive"`.
-   `deactivated_at`: A timestamp set when a key is deactivated, used for cooldown logic. `null` if active.

### Rate Limits (`src/config.rs`)

Hardcoded rate limits can be adjusted in the `src/config.rs` file.

```rust
pub const RPM_LIMIT: u32 = 2;       // Requests Per Minute per key
pub const RPD_LIMIT: u32 = 50;      // Requests Per Day per key
pub const TPM_LIMIT: u32 = 125000;  // Total Processed per Minute across all keys
pub const KEY_COOLDOWN_SECONDS: i64 = 300; // 5 minutes
```

## API Endpoints

### Get Next API Key

-   **Endpoint**: `GET /next`
-   **Description**: Retrieves the next available API key based on the selected mode.
-   **Query Parameters**:
    -   `mode` (optional): `auto` (default) or `random`.
-   **Response**:
    -   `200 OK`: `{"api_key": "some-key-string"}`
    -   `429 TOO_MANY_REQUESTS`: If the global TPM limit is reached.
    -   `503 SERVICE_UNAVAILABLE`: If no keys are currently available.
-   **Example**:
    ```sh
    curl "http://localhost:8080/next?mode=auto"
    ```

### Key Management

| Method | Endpoint              | Description                               | Request Body                |
| :----- | :-------------------- | :---------------------------------------- | :-------------------------- |
| `POST` | `/add`                | Adds a single new API key.                | `{"key": "new-key"}`        |
| `POST` | `/add_bulk`           | Adds multiple new API keys.               | `{"keys": ["key1", "key2"]}` |
| `DELETE`| `/delete/:key`        | Permanently removes an API key.           | -                           |
| `POST` | `/deactivate/:key`    | Temporarily disables a key (starts cooldown).| -                           |
| `POST` | `/reactivate/:key`    | Manually re-enables a key.                | -                           |
| `GET`  | `/keys`               | Lists all API keys and their status.      | -                           |

### Statistics

| Method | Endpoint | Description                              |
| :----- | :------- | :--------------------------------------- |
| `GET`  | `/stats` | Gets usage stats (total, active keys).   |

## Next.js Helper (`api_key.ts`)

A helper function is provided for easy frontend integration. It includes a simple in-memory cache to prevent over-fetching.

```typescript
// api_key.ts

let apiKeyCache: { key: string; timestamp: number } | null = null;

export async function getApiKey(
  mode: "auto" | "random" = "auto"
): Promise<string> {
  const now = Date.now();

  // Return from cache if valid (500ms)
  if (apiKeyCache && now - apiKeyCache.timestamp < 500) {
    return apiKeyCache.key;
  }

  try {
    const response = await fetch(
      `http://localhost:8080/next?mode=${mode}`
    );
    if (!response.ok) {
      throw new Error("Failed to fetch API key");
    }
    const data = await response.json();
    const apiKey = data.api_key;

    apiKeyCache = { key: apiKey, timestamp: now };
    return apiKey;
  } catch (error) {
    console.error("Error fetching API key:", error);
    // Provide a fallback or handle the error appropriately
    return "default-fallback-key";
  }
}

