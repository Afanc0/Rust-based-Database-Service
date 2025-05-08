# Rust-based Database Service

A Rust-powered service for interacting with a MongoDB database. This service allows you to connect to MongoDB, perform basic database operations, and handle configuration through environment variables.

## Features

- **MongoDB Integration**: Easily connects to a MongoDB instance using the provided URI.
- **Environment Variables**: Configuration such as the MongoDB URI is managed via environment variables using a `.env` file.
- **Error Handling**: Provides basic error handling and logging when database connection fails or when the environment variable is missing.

## Setup

### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install).
- Install [MongoDB](https://www.mongodb.com/try/download/community) or use a cloud-based instance (e.g., MongoDB Atlas).
- Set up a `.env` file with your MongoDB URI.

### Installation

1. Clone the repository:
   git clone <your-repository-url>
   cd <your-project-directory>

2. Install dependencies:
   cargo build

3. Create a `.env` file in the root directory and add your MongoDB URI:
   MONGO_URI="<connection string>"

4. Use main.rs to invoked the services:
  ```rust
    db_client::init_client().await?; 
    let docs = db_client::find_many("cats", doc! {}, "animals").await?;
    println!("Found docs: {:?}", docs);
  ```

5. Run the service:
   cargo run

## Usage

- The service will automatically connect to the MongoDB instance using the URI specified in the `.env` file.
- On success, it will print a message indicating the successful connection to the database.
- If an error occurs (e.g., if the environment variable is missing or the database is unreachable), it will print an error message.


