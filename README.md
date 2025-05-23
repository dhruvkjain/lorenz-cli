# Lorenz Attractor Terminal Plotter

[<img alt="github" src="https://img.shields.io/badge/github-dhruvkjain/lorenz_cli-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dhruvkjain/lorenz-cli)
[<img alt="crates.io" src="https://img.shields.io/crates/v/lorenz-cli.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/lorenz-cli)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-lorenz_cli-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/lorenz-cli)
![Crates.io](https://img.shields.io/crates/d/lorenz-cli)

This project visualizes the **Lorenz Attractor** in the terminal using **Ratatui**. The attractor is dynamically plotted in real-time, allowing users to choose different projection axes.

## Features
- Real-time plotting of the **Lorenz Attractor**
- Supports different 2D projections (**XY, XZ, YZ**)
- Dynamically adjusts the graph bounds
- **Animated rendering** using terminal graphics
- Exit the visualization anytime by pressing **'q'** or **Esc**

## Installation
### **1. Install Rust (if not already installed)**
Ensure you have Rust installed. If not, install it using:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Then restart your terminal and verify the installation:
```sh
rustc --version
cargo --version
```

### **2. Installing from Crates.io**
You can install crate directly too from:
```sh
cargo install lorenz-cli
```
Run it:
```sh
lorenz-cli xz
```

## Example Output
![image](https://github.com/user-attachments/assets/420bcce7-493f-4976-9366-e4a069805ed5)

(*Rendered using Braille characters in Ratatui*)

## Development
### **1. Install Rust (if not already installed)**
Ensure you have Rust installed. If not, install it using:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Then restart your terminal and verify the installation:
```sh
rustc --version
cargo --version
```

### **2. Clone the Repository**
```sh
git clone https://github.com/dhruvkjain/lorenz-cli.git
cd lorenz-cli
```

### **3. Build the Project**
```sh
cargo build --release
```

### **4. Run the Lorenz Attractor**
To run with the default projection (**XY**):
```sh
cargo run --release
```
Or specify an axis:
```sh
cargo run --release -- xz  # Options: xy, xz, yz
```

## License
This project is licensed under the **MIT License**.

## Author
[Dhruv Jain](https://github.com/dhruvkjain)

