# O~2~ - A Breath of Fresh Air for Rust Web Development

## TBD: Crate is in development, not yet released. It does not work now.


Fixes the fragmented Rust web development space by using composable traits to provide a consistent and ergonomic API for web development. Allows you to build web applications using a set of composable traits that can be mixed and matched to suit your needs.
# O<sub>2<sub/> - A Breath of Fresh Air for Rust Web Development
# Features
- **Composable Traits**: Build your web application using composable traits that can be mixed and matched.
- **Ergonomic API**: Provides a consistent and ergonomic API for web development.
- **Type Safety**: Leverages Rust's type system to ensure safety and correctness.
- **Asynchronous**: Built on top of async Rust, allowing for high-performance web applications.
- **Modular Design**: Easily extendable and customizable to fit your specific needs.
# Getting Started
To get started with O~2~, add it to your `Cargo.toml`:

```toml
[dependencies]
o2 = "0.01"
```

Then, you can start building your web application by importing the necessary traits and types from O~2~.
# Example
```rust
use o2::prelude::*;
#[tokio::main]
async fn main() {
    let app = App::new()
        .route("/", get(|| async { "Hello, O~2~!" }))
        .route("/hello", get(|| async { "Hello, World!" }));

    app.run("     example.com:8080").await.unwrap();
}
```
# Contributing
We welcome contributions to O~2~! If you have ideas, suggestions, or bug fixes, please open an issue or submit a pull request.
# License
O~2~ is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
# Contact
For questions, suggestions, or feedback, please reach out to us at chris@striveweekly.com or open an issue on GitHub.
# Acknowledgements
O~2~ is inspired by the need for a more cohesive and ergonomic web development experience in Rust. We appreciate the contributions of the Rust community and the developers of existing web frameworks that have paved the way for O~2~.
# Roadmap
- [ ] Implement more composable traits for common web development tasks.
- [ ] Improve documentation and examples.
- [ ] Add support for more web standards and protocols.
- [ ] Explore integration with existing Rust web frameworks.
# Community
Join our community on [Discord](https://discord.gg/example) to discuss O~2~, share your experiences, and get help from other developers.
# Changelog
See the [CHANGELOG](CHANGELOG.md) for a detailed list of changes and updates to O~2~.
# Support
If you find O~2~ useful, consider supporting the project by starring it on GitHub or sharing it with your friends and colleagues. Your support helps us continue to improve and maintain the project.
# Disclaimer
O~2~ is an experimental project and may not be suitable for production use. Use at your own risk. We are continuously working to improve the library, and your feedback is invaluable in shaping its future.
# O~2~ - A Breath of Fresh Air for Rust Web Development

