# Build Your Own Redis
![Project Status](https://img.shields.io/badge/status-in%20progress-yellow)
![Language](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-green)
![CI/CD](https://img.shields.io/badge/CI%2FCD-Jenkins-red)

## Overview
This project is a from-scratch implementation of Redis in Rust. The goal is to understand Redis's core functionality by implementing its key features while leveraging Rust's safety and performance characteristics.

**Note:** All code in this repository is written by hand, with no copy-pasting from existing implementations or AI-generated code. This project represents my learning journey to understand Redis at a fundamental level while improving my Rust programming skills.

## Learning Objectives
This project aims to deepen my understanding of systems programming through hands-on implementation. Key learning objectives include:
- **Networking fundamentals**: Understanding TCP/IP, socket programming, and client-server architecture in Rust
- **Protocol design**: Implementing the RESP (Redis Serialization Protocol) from specifications
- **Concurrent programming**: Leveraging Rust's async/await and ownership model for safe concurrent connections
- **In-memory database concepts**: Designing efficient data structures for key-value storage
- **System design**: Creating a modular, maintainable architecture
- **Performance optimization**: Building efficient data handling and processing systems
- **CI/CD practices**: Implementing a professional-grade continuous integration pipeline with Jenkins
- **DevOps skills**: Automating build, test, and deployment processes to a private development server

## Development Approach
I'm taking an incremental, test-driven approach to this project:
1. **Research**: Understanding Redis concepts before implementation
2. **Incremental development**: Building one feature at a time, starting with core functionality
3. **Testing**: Creating comprehensive unit and integration tests for each component
4. **Automated CI/CD**: Using Jenkins to automate builds, tests, and deployment
5. **Refactoring**: Continuously improving code quality and architecture
6. **Documentation**: Documenting design decisions and implementation details

## Project Roadmap
- [ ] Stage 1: TCP server that responds to PING
- [ ] Stage 2: RESP protocol parser and serializer
- [ ] Stage 3: Handling multiple clients concurrently with Rust's async/await
- [ ] Stage 4: Basic Redis commands (GET, SET, ECHO)
- [ ] Stage 5: Key expiration functionality
- [ ] Stage 6: Additional commands (DEL, EXISTS)
- [ ] Stage 7: Replication functionality
- [ ] Stage 8: CI/CD pipeline implementation with Jenkins
- [ ] Stage 9: Automated performance benchmarking
- [ ] Stage 10: Deployment to private development server

## Building and Running
### Prerequisites
- Rust (stable, 2021 edition or newer)
- Cargo
- Jenkins (for CI/CD pipeline)
- Docker (for containerized testing and deployment)

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run --release
```

### Run with port
```bash
cargo run --release -- --port 6379
```

### Test
```bash
cargo test
```

### Test with redis-cli
```bash
redis-cli ping
```

## Core Concepts to Explore
### TCP Networking in Rust
Redis operates as a TCP server, requiring an understanding of:
- Tokio or async-std for asynchronous networking
- Rust's ownership model for safe connection management
- Efficient buffer handling with Rust's memory safety guarantees

### RESP Protocol
The Redis Serialization Protocol (RESP) is a simple text-based protocol that:
- Serializes different data types (strings, errors, integers, arrays)
- Uses a prefix notation to distinguish types
- Supports both request and response formatting

### In-Memory Data Storage
Redis is fundamentally an in-memory database, which involves:
- Rust collections and custom data structures for efficient storage
- Memory management considerations with Rust's ownership model
- Expiration mechanisms for time-based key removal

### Concurrency
A production-ready Redis implementation in Rust leverages:
- Async/await for efficient handling of multiple client connections
- Rust's ownership and borrowing rules for thread safety
- Lock-free data structures where appropriate

### CI/CD Pipeline
The project incorporates a Jenkins pipeline for robust continuous integration:
- Automated building and testing on code push
- Performance benchmarking against baseline metrics
- Deployment to a private development server
- Regression testing to ensure compatibility

## Jenkins CI/CD Implementation
This project uses Jenkins to automate the development workflow:

### Pipeline Stages
1. **Code Checkout**: Pulls the latest code from the repository
2. **Build**: Compiles the codebase with Rust's cargo system
3. **Unit Tests**: Runs test suite to verify functionality
4. **Integration Tests**: Tests Redis protocol compatibility
5. **Performance Tests**: Benchmarks against previous versions
6. **Security Audit**: Uses cargo-audit to check dependencies
7. **Code Quality**: Runs clippy and rustfmt checks
8. **Deployment**: Deploys to private development server

### Development Benefits
- Immediate feedback on code changes
- Consistent build environment
- Automatic regression testing
- Performance tracking across versions
- One-click deployment for testing
- Automated code quality enforcement

### Infrastructure
- Dedicated private development server with optimized resources
- Integration with version control for automated triggers
- Secure access via private network
- Containerized deployment for isolation

## Learning Resources
- [Redis Protocol Specification](https://redis.io/topics/protocol)
- [Redis Command Reference](https://redis.io/commands)
- [Redis Internals](https://redis.io/topics/internals)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Documentation](https://tokio.rs/tokio/tutorial)
- [Jenkins Pipeline Documentation](https://www.jenkins.io/doc/book/pipeline/)
- [Continuous Integration Best Practices](https://martinfowler.com/articles/continuousIntegration.html)
  
## License
This project is licensed under the MIT License.
