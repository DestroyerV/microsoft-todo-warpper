# Microsoft To Do Wrapper

This project is a wrapper for the Microsoft To Do application, built using Tauri.

## ✨ Features

- [x] Custom CSS injection
- [x] System tray integration
- [x] Autostart on system boot
- [x] Window state persistence
- [x] Notifications

## 🛠️ Building

### 📋 Prerequisites

- [Node.js](https://nodejs.org/)
- [Rust](https://www.rust-lang.org/)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/)

### 🛠️ Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/microsoft-todo-wrapper.git
   cd microsoft-todo-wrapper
   ```

2. Install dependencies:

   ```bash
   pnpm install
   ```

3. Build the project:

   ```bash
   pnpm tauri build
   ```

4. Run the project:

   ```bash
   pnpm tauri dev
   ```

## 📂 Project Structure

- `src/`: Contains the source code for the application.
- `src-tauri/`: Contains the Tauri-specific code and configuration.
- `public/`: Contains static assets.
- `dist/`: Contains the build output.

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
