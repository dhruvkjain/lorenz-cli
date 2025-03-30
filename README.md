# Lorenz Attractor Terminal Plotter

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

