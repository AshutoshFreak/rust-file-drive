# Rust File Sharing Service

A web-based service built in Rust that allows users to upload files and retrieve them via unique URLs. The service uses Actix Web for the REST API, Diesel ORM with SQLite for storage, and supports asynchronous file operations.

## Prerequisites

- Rust and Cargo (latest stable version)
- SQLite3
- Diesel CLI

## Setup Instructions

### **Install Required Dependencies**

```bash
# Install SQLite (Ubuntu/Debian)
sudo apt-get install sqlite3 libsqlite3-dev

# Install Diesel CLI with SQLite support
cargo install diesel_cli --no-default-features --features sqlite

```



### **Set Up Environment**

Create a .env file in the project root:

```bash
echo "DATABASE_URL=sqlite:files.db" > .env
```

### **Set Up Database**

```bash
# Initialize Diesel
diesel setup

# Run migrations
diesel migration run
```

### **Build and Run the Server**

```bash
cargo run
```

The server will start on http://localhost:8080

## API Usage
### Upload a File
To upload a file, use the following curl command:

```bash
curl -F "file=@path/to/your/file.txt" http://localhost:8080/api/upload
```

The response will look like:

```json
{
    "id": "unique-file-id",
    "url": "/files/unique-file-id"
}
```

## Download a File
Using the file ID from the upload response:

```bash
curl http://localhost:8080/api/files/unique-file-id -o downloaded_file

Replace:

    unique-file-id with the ID received from upload
    downloaded_file with your desired output filename
```

## Project Structure

```text
.
├── Cargo.toml
├── .env
├── migrations/
│   └── TIMESTAMP_create_files/
│       ├── up.sql
│       └── down.sql
└── src/
    ├── main.rs
    ├── models.rs
    ├── schema.rs
    ├── handlers.rs
    └── db.rs
```

## Database Schema
The service uses SQLite with the following schema:

```sql
CREATE TABLE files (
    id TEXT PRIMARY KEY NOT NULL,
    filename TEXT NOT NULL,
    content_type TEXT NOT NULL,
    size INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    data BLOB NOT NULL
);
```

## Error Handling

- If a file is not found, the server returns a 404 status code
- If no file is provided during upload, a 400 Bad Request is returned
- Database connection errors will return a 500 Internal Server Error


## Dependencies
Key dependencies used in this project:

- actix-web: Web framework
- diesel: ORM and Query Builder
- tokio: Async runtime
- serde: Serialization/Deserialization
- uuid: Unique ID generation
