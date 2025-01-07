

# Midgard Data Fetching and API Documentation

## Overview

This Rust application is designed to fetch data from the Midgard API and store it in a MongoDB database. The data is then served through RESTful APIs built with Actix Web, allowing users to access various data points related to Midgard, including:

1. **Depth History**
2. **Swaps**
3. **Earnings**
4. **Rune Pool Units & Counts**

The application supports optional query parameters that allow customization of the fetched data. The project is structured in a way that separates concerns into distinct modules, promoting maintainability and scalability.

## Folder Structure

### `src/`
This is the main folder for the application's code.

- **`db/`**: Contains the code responsible for interacting with the MongoDB database, such as fetching data from MongoDB and handling the connection.
  
- **`handlers/`**: Responsible for processing incoming HTTP requests. Each handler maps to one of the API endpoints and handles the logic to fetch data from MongoDB or interact with the database layer.

- **`utils/`**: Contains utility functions for data manipulation, such as sorting, grouping, and handling date/time conversions. Also includes functions for configuration management.

- **`models/`**: Defines the data models that correspond to the structure of the data being fetched and stored. These models are used when interacting with the database or responding to API requests.

- **`services/`**: Responsible for the logic of fetching data from the database service. This service layer abstracts the database interactions, making it easy to reuse the code for multiple parts of the application.

- **`state/`**: Holds the application state, such as global variables and configurations that need to be shared across various parts of the application.

- **`routes/`**: Contains the routing definitions for the Actix Web server. This folder sets up the different API endpoints and connects them to their respective handlers.

- **`error/`**: Contains error handling logic related to MongoDB operations, ensuring graceful error responses when MongoDB interactions fail.

## Features

- **Data Fetching**: Fetches data from the Midgard API and stores it in a MongoDB database.
- **Actix Web API Endpoints**: Provides four RESTful API endpoints that allow querying of stored data.
- **Query Parameters**: The API supports optional query parameters to customize the data being fetched, such as `date_range`, `sort_by`, `order`, and `limit`.

## API Endpoints

The application exposes the following endpoints:

### 1. Depth History
- **Endpoint**: `/depths`
- **Query Parameters**:
  - `date_range` (e.g., `2024-11-01,2023-10-02`)
  - `sort_by` (e.g., `asset_price`)
  - `order` (e.g., `asc` or `desc`)
  - `limit` (e.g., `24`)

- **Sample Query**:
  ```bash
  curl -X GET 'http://localhost:3000/api/depths?date_range=2024-11-01,2023-10-02'
  ```
   ```bash
  curl -X GET 'http://localhost:3000/api/earnings'
  ```
    ```bash
  curl -X GET 'http://localhost:3000/api/swaps?sort_by=runePriceUSD&limit=25&order=desc'
  ```
    ```bash
  curl -X GET 'http://localhost:3000/api/runepool?limit=25&order=desc'
  ```
### 2. Swaps
- **Endpoint**: `/swaps`
- **Query Parameters**:
  - `date_range` (e.g., `2024-11-01,2023-10-02`)
  - `sort_by` (e.g., `asset_price`)
  - `order` (e.g., `asc` or `desc`)
  - `limit` (e.g., `24`)

### 3. Earnings
- **Endpoint**: `/earnings`
- **Query Parameters**:
  - `date_range` (e.g., `2024-11-01,2023-10-02`)
  - `sort_by` (e.g., `asset_price`)
  - `order` (e.g., `asc` or `desc`)
  - `limit` (e.g., `24`)

### 4. Rune Pool Units & Counts
- **Endpoint**: `/runepools`
- **Query Parameters**:
  - `date_range` (e.g., `2024-11-01,2023-10-02`)
  - `sort_by` (e.g., `asset_price`)
  - `order` (e.g., `asc` or `desc`)
  - `limit` (e.g., `24`)

## Setup Instructions

1. **Clone the repository**:
   ```bash
   git clone <repository-link>
   ```

2. **Navigate to the project directory**:
   ```bash
   cd <project-directory>
   ```

3. **Install dependencies**:
   ```bash
   cargo build
   ```

4. **Run the application**:
   ```bash
   cargo run
   ```

5. **Access the APIs**:
   Open your browser or use `curl` to access the APIs at `http://localhost:<port>`.

## Prerequisites

- **Rust**: Make sure Rust is installed. You can install it from [Rust's official website](https://www.rust-lang.org/).
- **MongoDB**: Ensure MongoDB is running locally or configure the connection string to use a remote MongoDB instance.
- **Actix Web**: Ensure the Actix Web crate is added to the `Cargo.toml` file for building RESTful APIs.

## Environment Variables

- **MONGODB_URI**: The MongoDB connection string.
- **DATABASE_NAME**: The MongoDB database name.
- **COLLECTION_NAME**: The MongoDB collection name.
- **SERVER_ADDR**: The address where the Actix Web server will run (e.g., `0.0.0.0:3000`).

## Error Handling

The application uses custom error handling, especially for MongoDB interactions. Errors such as connection failures, query issues, or missing data are caught and appropriate error messages are returned to the user.

## Best Practices

- **Use Query Parameters Wisely**: Not all query parameters can be used together at the same time. Be mindful of the potential conflicts when using `date_range`, `sort_by`, `order`, and `limit`.
- **Efficient Data Fetching**: For large datasets, be cautious with the `limit` parameter to avoid overwhelming the database or causing long API response times.
- **Error Handling**: Ensure that all potential error states are handled gracefully, particularly with MongoDB connections or when fetching non-existent data.
- **Testing**: Always test the API with different combinations of query parameters to ensure the application behaves as expected under different conditions.

## Acknowledgments

- **Midgard API**: Provides the data that the application fetches and processes.
- **Actix Web**: Used for building the RESTful API.
- **Postman**: Used for creating the API documentation.

