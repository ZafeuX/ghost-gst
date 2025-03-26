# Ghost (gst): Ultra-Fast, Secure Remote Access

*Ghost (gst)* is a cutting-edge remote access tool engineered for unparalleled speed, security, and efficiency. Built in *Rust*, it aims to alter remote access by providing a compile-time optimised, low-latency alternative to traditional SSH solutions.

## Project Goals

- **Lightning-Fast Transport**: Achieve significantly faster connection establishment and data transfer compared to existing solutions.
- **State-of-the-Art Security**: Implement robust authentication mechanisms, including *FIDO2* and *Passkey* support, to ensure top-tier protection.
- **Efficient Multiplexing**: Enable highly efficient multi-channel sessions with minimal overhead for optimal performance.

## Planned Features

- **Compile-Time Configuration**: Maximise performance by compiling all configuration options directly into the binary.
- **Multiplexing-First Architecture**: Support multiple channels per connection for superior resource utilisation.
- **Zero-Copy Data Forwarding**: Drastically reduce memory allocations for ultra-fast data transfer.
- **Lock-Free Ring Buffers**: Ensure efficient, concurrent data handling for smooth operations.
- **Advanced Authentication**: Incorporate *FIDO2* and *Passkey* support for enhanced security measures.
- **Ghost Daemon**: Offer a lightweight, high-performance SSH daemon alternative for server-side deployment.
- **WebAssembly Integration**: Execute remote commands securely within a *WASM* sandbox for added protection.

## Early Access

*Ghost (gst)* is currently in **early access**. I am actively developing core features and improving stability. Early adopters can expect:

- Frequent updates and improvements
- Potential breaking changes as the architecture is refined
- Opportunities to shape the project's direction through feedback and contributions

## Contributing

I welcome contributions from the community! To contribute:

1. **Fork** the repository
2. **Create a feature branch**
3. **Make your changes**, ensuring comprehensive tests and documentation
4. **Submit a pull request** to the relevant feature branch

Please refer to our [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## Building from Source

To build *Ghost (gst)*, ensure you have *Rust* installed on your system. Then, execute the following command:

```rust
cargo build --release
```

## License

This project is licensed under the Mozilla Public License 2.0. For full details, please refer to the [LICENSE](LICENSE) file in the repository. For an explanation of why this license was chosen, see the [LICENSE_NOTES.md](LICENSE_NOTES.md) file.
