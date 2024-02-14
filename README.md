# Block Constructor

This is a Rust project designed to construct blocks of transactions from a mempool CSV file. It parses transaction data, handles parent transactions, and generates an output file containing the constructed blocks.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)

## Installation

Explain how to install and set up your project. Include any dependencies and instructions for installing them.

```sh
# Clone the repository
git clone git@github.com:isaack-njama/block-constructor.git

# Navigate to the project directory
cd block-constructor

# Build the project
cargo build
```

## Usage

```sh
# Run the project
cargo run
```

## Testing

```sh
# Run tests
cargo test
```

Note:
Tests are currently failing and the project is a work in progress. We are actively working on resolving the issues and improving test coverage. Feel free to check back later for updates.

## Contributing

Contributions are welcome! If you'd like to contribute to this project, feel free to fork the repository, make your changes, and submit a pull request.

## Reasoning, Design Decisions, and Trade-offs

Block Constructor was designed with simplicity, modularity, and performance in mind. Here are some key aspects of the reasoning, design decisions, and trade-offs:

Simplicity: The project aims to provide a straightforward solution for constructing blocks of transactions from a mempool CSV file. It follows Rust's idiomatic style and uses standard libraries for file I/O and data processing.

Modularity: The code is structured into modular components, such as the MempoolTransaction struct and its associated methods. This modular design allows for easy maintenance, testing, and future enhancements.

Performance: While simplicity and modularity are prioritized, performance considerations are also taken into account. For example, the use of data structures like HashSet helps to optimize transaction handling and avoid writing duplicate entries to the output file.

Error Handling: Error handling is implemented to provide informative error messages and gracefully handle unexpected situations, such as file I/O errors or malformed input data.

Trade-offs: One trade-off is the reliance on synchronous file I/O operations, which may impact performance when dealing with large CSV files. Asynchronous I/O could be considered for future optimizations, but it might add complexity to the codebase.

## Contact

For any inquiries or feedback, please contact me at <isaacknjama@proton.me>

## License

This project is licensed under the [MIT License](LICENSE).
