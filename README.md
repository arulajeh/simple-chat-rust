# Simple Chat Rust

A lightweight WebSocket-based chat server built with Rust, designed for real-time messaging with multiple users.

## Features

- **Real-time messaging** - WebSocket-based communication for instant message delivery
- **Multi-user support** - Multiple users can connect and chat simultaneously
- **HTML sanitization** - All messages are automatically sanitized to prevent XSS attacks
- **User identification** - Users connect with unique user IDs via query parameters
- **Broadcast messaging** - Messages are broadcasted to all connected users
- **Deployment ready** - Includes Docker and Fly.io configuration

## Technology Stack

- **Rust** - High-performance systems programming language
- **Tokio** - Asynchronous runtime for Rust
- **tokio-tungstenite** - WebSocket implementation
- **Ammonia** - HTML sanitization library
- **Docker** - Containerization
- **Fly.io** - Cloud deployment platform

## Prerequisites

- Rust 1.70+ (2021 edition)
- Cargo package manager

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd simple-chat-rust
```

2. Build the project:
```bash
cargo build --release
```

## Usage

### Running Locally

Start the server:
```bash
cargo run
```

The server will start on `0.0.0.0:8080` and accept WebSocket connections.

### Connecting to the Chat

Connect to the WebSocket server with a user ID:
```
ws://localhost:8080?user_id=your_username
```

Example using JavaScript:
```javascript
const socket = new WebSocket('ws://localhost:8080?user_id=alice');

socket.onopen = function(event) {
    console.log('Connected to chat');
};

socket.onmessage = function(event) {
    console.log('Message:', event.data);
};

socket.send('Hello, everyone!');
```

## Deployment

### Docker

Build and run with Docker:
```bash
docker build -t simple-chat-rust .
docker run -p 8080:8080 simple-chat-rust
```

### Fly.io

The project includes Fly.io configuration. To deploy:

1. Install the Fly CLI
2. Login to Fly.io: `fly auth login`
3. Deploy: `fly deploy`

The app is configured to run in the Singapore region (`sin`) with automatic scaling.

## API

### WebSocket Connection

- **Endpoint**: `ws://host:port`
- **Required Query Parameter**: `user_id` - Unique identifier for the user
- **Example**: `ws://localhost:8080?user_id=alice`

### Message Format

- Messages are sent as plain text
- All HTML content is automatically sanitized
- Messages are broadcasted to all connected users in the format: `{user_id}: {message}`

## Security Features

- **HTML Sanitization**: All user input is sanitized using the Ammonia library
- **URL Decoding**: Proper handling of URL-encoded content
- **Input Validation**: User ID validation during WebSocket handshake

## Development

### Project Structure

```
src/
├── main.rs          # Main server implementation
Cargo.toml           # Dependencies and project configuration
Dockerfile           # Docker containerization
fly.toml            # Fly.io deployment configuration
```

### Key Dependencies

- `tokio` - Async runtime with full features
- `tokio-tungstenite` - WebSocket server implementation
- `futures-util` - Stream and sink utilities
- `ammonia` - HTML sanitization
- `urlencoding` - URL decoding utilities

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is open source. Please check the repository for license details.