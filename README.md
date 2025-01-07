# Rust Application for Midgard Data Fetching and API

## Overview

This Rust application fetches data from the Midgard API and stores it in a MongoDB database. The application provides four API endpoints using Actix Web to serve the stored data:

1. **Depth History**
2. **Swaps**
3. **Earnings**
4. **Rune Pool Units & Counts**

All endpoints support optional query parameters to customize the fetched results.

## Deployment

**Deployed Application:** [Add Deployment Link Here]

## API Documentation

Detailed API documentation is available and has been created using Postman. You can find the complete documentation at the following link:

**Postman Documentation:** [Add Postman Documentation Link Here]

## Features

- Fetches data from four endpoints of the Midgard API:
  - **Depth History**
  - **Swaps**
  - **Earnings**
  - **Rune Pool Units & Counts**
- Stores the fetched data in a MongoDB database.
- Exposes RESTful API endpoints using Actix Web to fetch the stored data with optional query parameters.

## API Endpoints

### 1. Depth History
**Endpoint:** `/depths`

**Query Parameters:**
- `date_range` (e.g., `2024-11-01,2023-10-02`)
- `sort_by` (e.g., `asset_price`)
- `order` (e.g., `asc` or `desc`)
- `limit` (e.g., `24`)
- `page` (e.g., `1`)

**Sample Query:**
```bash
curl -X GET 'http://localhost:8080/depths?date_range=2024-11-01,2023-10-02&sort_by=asset_price&order=desc&limit=24&page=1'
```

### 2. Swaps
**Endpoint:** `/swaps`

**Query Parameters:**
- `date_range` (e.g., `2024-11-01,2023-10-02`)
- `sort_by` (e.g., `asset_price`)
- `order` (e.g., `asc` or `desc`)
- `limit` (e.g., `24`)
- `page` (e.g., `1`)

### 3. Earnings
**Endpoint:** `/earnings`

**Query Parameters:**
- `date_range` (e.g., `2024-11-01,2023-10-02`)
- `sort_by` (e.g., `asset_price`)
- `order` (e.g., `asc` or `desc`)
- `limit` (e.g., `24`)
- `page` (e.g., `1`)

### 4. Rune Pool Units & Counts
**Endpoint:** `/runepool_units_counts`

**Query Parameters:**
- `date_range` (e.g., `2024-11-01,2023-10-02`)
- `sort_by` (e.g., `asset_price`)
- `order` (e.g., `asc` or `desc`)
- `limit` (e.g., `24`)
- `page` (e.g., `1`)

## Prerequisites

### 1. Rust
Ensure Rust is installed. You can install it from [Rust's official website](https://www.rust-lang.org/).

### 2. MongoDB
Install and configure MongoDB. For local development, ensure MongoDB is running on the default port or update the connection string in the application code.

### 3. Actix Web
The application uses Actix Web for creating RESTful APIs. Ensure the Actix Web crate is included in your `Cargo.toml` file.

## Setup Instructions

1. Clone the repository:
   ```bash
   git clone <repository-link>
   ```

2. Navigate to the project directory:
   ```bash
   cd <project-directory>
   ```

3. Install dependencies:
   ```bash
   cargo build
   ```

4. Run the application:
   ```bash
   cargo run
   ```

5. Access the APIs at `http://localhost:<port>`.

## Environment Variables

- `MONGO_URI`: MongoDB connection string.
- `PORT`: Port for the application to run.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Acknowledgments

- **Midgard API**: Used for data fetching.
- **Actix Web**: For building RESTful APIs.
- **Postman**: For API documentation.

---
Feel free to customize this `README.md` file as per your project's specific details and requirements.
