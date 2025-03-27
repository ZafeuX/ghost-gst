# Changelog

## [0.3.0] - 2025-03-27
- Implemented backoff strategy for connection attempts in `connection.rs`
- Added compile-time configuration for connection parameters using const generics
- Updated to use const generics for improved performance and flexibility
- Fixed deprecation warnings for rand crate usage [Version 0.9.0]
- Created a basic server component (`server`) for testing client functionality

## [0.2.0] - 2025-03-26
- Implemented custom CLI parser
- Added configuration module for loading settings from a file
- Created a basic connection module for TCP communication
- Improved error handling for connection attempts [Backoff Method has yet to be implemented.]

## [0.1.0] - 2025-03-26
- Initial project setup
- Basic project structure
