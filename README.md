# SynapseDB

SynapseDB is a hybrid database engine that combines the performance characteristics of Rust with the concurrency strengths of Go. This system is designed to deliver fast, reliable, and scalable database operations for modern data workloads.

## Overview

SynapseDB leverages a dual-language architecture where the core storage engine and query processor are implemented in Rust for optimal performance and memory safety, while the networking layer and API server are built in Go to take advantage of its excellent concurrency primitives. The system bridges these components using C FFI (Foreign Function Interface), enabling seamless interoperation between the two languages.

One of the distinguishing features of SynapseDB is its integrated AI-powered natural language to SQL translation capability, allowing users to interact with the database using plain English queries that are automatically converted to SQL statements.

## Architecture

### Core Components

#### Storage Engine (Rust)
The storage engine is implemented in Rust and provides:
- **Page-based storage**: Data is organized into pages with configurable row limits
- **Persistent storage**: Tables and metadata are serialized to disk using binary encoding
- **Table management**: Metadata tracking, page allocation, and row management
- **Type system**: Support for INTEGER, TEXT, PHONENUMBER, and EMAIL data types

Location: `internal/engine/src/storage/`

#### Query Processor (Rust)
The query processing layer handles:
- **SQL parsing**: Tokenization and parsing of SQL statements
- **Query execution**: Support for CREATE, INSERT, and SELECT operations
- **AST representation**: Abstract syntax tree for query representation
- **Expression evaluation**: Support for WHERE clauses with binary operators (EQ, NEQ, GT, LT, GTE, LTE)

Location: `internal/engine/src/query_processor/`

#### FFI Bridge (Rust)
The Foreign Function Interface layer exposes Rust functions to C:
- **C-compatible functions**: `execute_sql()` and `free_string()` for memory-safe string handling
- **Go integration**: Enables Go code to call Rust functions seamlessly

Location: `ffi/src/lib.rs`

#### API Server (Go)
The networking layer provides:
- **TCP server**: Listens on port 4538 for client connections
- **Connection handling**: Concurrent request processing using goroutines
- **AI integration**: Natural language to SQL conversion via OpenRouter API
- **Query execution**: Delegates SQL execution to the Rust engine via FFI

Location: `api/main.go`

#### Interactive Shell (Go)
A command-line interface for interacting with the database:
- **REPL interface**: Read-Eval-Print Loop for iterative query execution
- **Connection management**: Automatic reconnection handling
- **Performance metrics**: Query latency display
- **User-friendly output**: Color-coded terminal output

Location: `cmd/shell/main.go`

#### AI SQL Generator (Go)
Natural language processing component:
- **OpenRouter integration**: Connects to OpenRouter API for LLM-based SQL generation
- **Prompt engineering**: Structured prompts for reliable SQL conversion
- **Output validation**: Ensures generated SQL is valid and executable
- **Model flexibility**: Configurable LLM models via environment variables

Location: `ai/v1/get_sql.go`

## Project Structure

```
SynapseDB/
├── ai/                          # AI-powered SQL generation
│   ├── v1/                      # Version 1 implementation
│   │   └── get_sql.go          # OpenRouter integration
│   └── v2/                      # Version 2 (future implementation)
│       └── src/
│           ├── nn/             # Neural network components
│           └── utils/          # Utility functions
├── api/                         # API server
│   ├── execute/                 # Query execution layer
│   │   ├── executor.go         # Go-Rust FFI bridge
│   │   └── engine.dll          # Compiled Rust engine
│   └── main.go                 # TCP server implementation
├── assets/                      # Static assets
│   └── synapse_thumb.png       # Project thumbnail
├── cmd/                         # Command-line applications
│   └── shell/
│       └── main.go             # Interactive shell
├── ffi/                         # Foreign Function Interface
│   ├── Cargo.toml              # Rust crate configuration
│   └── src/
│       └── lib.rs              # C-compatible Rust functions
├── internal/                    # Internal packages
│   └── engine/                 # Core database engine
│       ├── Cargo.toml          # Rust dependencies
│       └── src/
│           ├── lib.rs          # Engine entry point
│           ├── query_processor/  # SQL parsing and execution
│           │   ├── ast.rs      # Abstract syntax tree
│           │   ├── executor.rs # Query execution logic
│           │   ├── mod.rs      # Module definitions
│           │   ├── parser.rs   # SQL parser
│           │   └── tokenizer.rs # Lexical analysis
│           └── storage/        # Storage engine
│               ├── disk.rs     # Disk I/O operations
│               ├── mod.rs      # Module definitions
│               ├── page.rs     # Page data structure
│               ├── row.rs      # Row data structure
│               └── table.rs    # Table management
├── src/                         # Data storage directory
│   └── storage/
│       └── tables/             # Persistent table data
│           └── users/          # Example table storage
│               ├── metadata.bin # Table metadata
│               └── page_0.bin  # Page data files
├── tests/                       # Test files
│   └── test.py                 # Python test suite
├── main.go                      # Application entry point
├── go.mod                       # Go module definition
├── go.sum                       # Go dependency checksums
└── README.md                    # This file
```

## Features

### Current Capabilities

- **SQL Query Support**:
  - `CREATE TABLE`: Define new tables with typed columns
  - `INSERT INTO`: Add rows to tables
  - `SELECT`: Query data with optional WHERE clauses

- **Data Types**:
  - INTEGER
  - TEXT
  - PHONENUMBER
  - EMAIL

- **Natural Language Interface**:
  - Convert plain English queries to SQL
  - Integrated with OpenRouter API
  - Configurable LLM models

- **Concurrent Processing**:
  - Multi-client support via goroutines
  - Non-blocking I/O operations

- **Persistent Storage**:
  - Binary serialization using bincode
  - Metadata persistence
  - Page-based file organization

### Planned Features

The project is under active development. Future enhancements may include:
- UPDATE and DELETE operations
- Advanced SQL features (JOINs, GROUP BY, ORDER BY)
- Indexing for performance optimization
- Transaction support
- Query optimization and execution planning
- Enhanced AI capabilities in v2

## Prerequisites

### Required Software

- **Go**: Version 1.22.5 or later
- **Rust**: Latest stable version with Cargo
- **C Compiler**: Required for building the FFI layer (MinGW on Windows, GCC on Linux/Mac)
- **OpenRouter API Key**: For AI-powered SQL generation (optional, for natural language queries)

### Platform Support

Currently tested and supported on:
- Windows (x86_64-pc-windows-msvc)

Cross-platform support may require additional configuration for the FFI layer.

## Installation

### Building from Source

1. **Clone the repository**:
   ```bash
   git clone https://github.com/DeveloperAromal/SynapseDB.git
   cd SynapseDB
   ```

2. **Build the Rust engine**:
   ```bash
   cd internal/engine
   cargo build --release
   ```

3. **Build the FFI layer**:
   ```bash
   cd ../../ffi
   cargo build --release --target x86_64-pc-windows-msvc
   ```

4. **Copy the compiled DLL** to the appropriate location:
   ```bash
   # Copy engine.dll to api/execute/ or update FFI LDFLAGS path
   ```

5. **Install Go dependencies**:
   ```bash
   go mod download
   ```

6. **Build the Go application**:
   ```bash
   go build -o synapsedb.exe main.go
   ```

### Configuration

Create a `.env` file in the project root with the following variables:

```env
OPENROUTER_API_KEY=your_api_key_here
MODEL=meta-llama/llama-3.1-8b-instruct:free
OPENROUTER_REFERER=your_referer_url (optional)
OPENROUTER_TITLE=your_app_title (optional)
```

The `MODEL` variable is optional and defaults to `meta-llama/llama-3.1-8b-instruct:free` if not specified.

## Usage

### Starting the Server

Run the main application to start both the API server and interactive shell:

```bash
go run main.go
```

Or use the compiled binary:

```bash
./synapsedb.exe
```

The server will start listening on port 4538, and the interactive shell will automatically connect.

### Interactive Shell

The shell provides a REPL interface for executing queries:

```
======================================================
-------------------- Synapse Shell -------------------
======================================================

Type exit to close the shell.

~> show all users
Latency: ~150ms
Received: SELECT * FROM users;

~> exit
Closing Synapse Shell...
```
<img width="1919" height="1021" alt="Screenshot 2025-10-29 155222" src="https://github.com/user-attachments/assets/df9cf58f-1e24-4086-86a9-aa264775f080" />

### Natural Language Queries

You can use natural language in the shell, which will be converted to SQL:

```
~> show all employees
~> add a new user named John with email john@example.com
~> find users where age is greater than 25
```

### Direct SQL Queries

SQL queries can also be executed directly:

```
~> CREATE TABLE users (id INTEGER, name TEXT, email EMAIL);
~> INSERT INTO users VALUES (1, 'John Doe', 'john@example.com');
~> SELECT * FROM users;
```

### Programmatic Access

You can connect to the database programmatically via TCP on port 4538:

```python
import socket

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.connect(('localhost', 4538))
query = "show all users in user table" # Your query in plain english
sock.send(query)
response = sock.recv(4096)
print(response.decode())
sock.close()
```

## Development

### Project Structure Principles

- **Separation of Concerns**: Storage engine (Rust) and networking (Go) are clearly separated
- **Language Strengths**: Rust for performance-critical operations, Go for concurrency
- **FFI Safety**: Memory-safe C-compatible interfaces between languages
- **Modularity**: Components are organized into logical modules

### Adding New Features

1. **SQL Operations**: Extend `query_processor/parser.rs` and `query_processor/executor.rs`
2. **Storage Features**: Modify `storage/` modules for new storage capabilities
3. **API Endpoints**: Extend `api/main.go` for new server functionality
4. **AI Integration**: Enhance `ai/v1/get_sql.go` or work on v2 implementation

### Testing

Run tests using the provided test suite:

```bash
python tests/test.py
```

## Performance Considerations

- **Rust Engine**: Compiled to native code for optimal performance
- **Page-based Storage**: Efficient memory usage and disk I/O
- **Concurrent Connections**: Go's goroutines handle multiple clients efficiently
- **FFI Overhead**: Minimal overhead when crossing language boundaries

## Limitations

- Currently in active development; not recommended for production use
- Limited SQL feature set (CREATE, INSERT, SELECT only)
- No transaction support
- No indexing or query optimization
- Windows-focused build configuration (may require adjustments for other platforms)

## License

[MIT License](https://github.com/DeveloperAromal/SynapseDB/blob/main/LICENSE)


## Acknowledgments

Built with:
- Rust - Systems programming language
- Go - Concurrent programming language
- OpenRouter - AI model API provider
- bincode - Binary serialization
- serde - Serialization framework


**Note**: This project is currently under active development. Features and APIs may change rapidly. This documentation reflects the current state of the codebase and may not cover all experimental features or recent changes.
