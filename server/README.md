# Blazingly Fast URL Shortener

## Overview

Blazingly Fast is a URL shortener service built with Rust using the Axum framework and SQLx for database interactions. It allows users to create short URLs that redirect to long URLs, track usage, and automatically clean up expired links.

## Features

- Create short URLs
- Retrieve original URLs
- Track usage counts
- Automatic cleanup of expired links

## Getting Started

### Prerequisites

- Rust (1.71 or later)
- PostgreSQL (version 12 or later)

### Installation

1. **Clone the repository:**

   ```bash
   git clone https://github.com/gurshaan17/blazingly-fast.git
   cd blazingly-fast
   ```

2. **Create the required table:**

   Run the following SQL command to create the `links` table:

   ```sql
   \c blazingly_fast
   CREATE TABLE links (
       id VARCHAR(10) PRIMARY KEY,
       target_url TEXT NOT NULL,
       expiration TIMESTAMP WITH TIME ZONE NOT NULL,
       usage_count BIGINT NOT NULL DEFAULT 0
   );
   ```

3. **Set environment variables:**

   Create a `.env` file in the root directory and add your database URL:

   ```plaintext
   DATABASE_URL=postgres://postgres:postgres@localhost:5432/blazingly_fast
   ```

4. **Build and run the server:**

   ```bash
   cargo run
   ```

   The server will start on `http://127.0.0.1:8080`.

## API Documentation

### Create Short URL

- **Endpoint:** `POST /url`
- **Request Body:**

  ```json
  {
      "target_url": "https://example.com"
  }
  ```

- **Response:**

  ```json
  {
      "id": "abc123",
      "target_url": "https://example.com",
      "expiration": "2023-10-01T12:00:00Z",
      "usage_count": 0
  }
  ```

- **Description:** Creates a new short URL. The `id` is generated automatically and the URL will expire in 24 hours.

### Retrieve Original URL

- **Endpoint:** `GET /url/:id`
- **Response:**

  - **Success:**

    ```json
    {
        "id": "abc123",
        "target_url": "https://example.com",
        "expiration": "2023-10-01T12:00:00Z",
        "usage_count": 0
    }
    ```

  - **Error (404 Not Found):**

    ```json
    {
        "error": "URL not found"
    }
    ```

- **Description:** Retrieves the original URL associated with the given short URL ID.

### Cleanup Expired Links

- **Endpoint:** `POST /cleanup`
- **Response:**

  ```json
  {
      "message": "Cleanup completed: 5 links deleted"
  }
  ```

- **Description:** Manually triggers the cleanup of expired links. This is usually done automatically every 5 minutes.
