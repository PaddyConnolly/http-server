# 🦀 Page Vault Server

A single-threaded HTTP server built solely using Rust's standard library for archiving webpage HTML.

## Usage

```
cargo run
```
## Endpoints

### `GET /health`
Health check endpoint.

### `POST /save`
Archive a webpage's HTML content.

**Custom Header:**
- `page-url` - The URL of the page being archived

**Body:**
- Raw HTML content
