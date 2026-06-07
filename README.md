# API Vault

A local desktop application for securely storing and managing API keys, built with Tauri 2 + Vue 3.

## Features

- **Secure Storage** - API keys are encrypted with AES-256-GCM and stored locally in SQLite
- **Password Protection** - Master password hashed with Argon2, with auto-lock on idle
- **Multi-Provider Support** - Built-in presets for popular AI services (OpenAI, Claude, etc.)
- **Search & Filter** - Quickly find keys by name, provider, or description
- **Copy to Clipboard** - Decrypt and copy keys with one click
- **Dark/Light Theme** - Supports system theme detection
- **Multi-language** - English and Chinese interface
- **Cross-platform** - Windows, macOS, Linux

## Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | Vue 3, Pinia, Naive UI, vue-i18n |
| Backend | Rust, Tauri 2 |
| Database | SQLite (rusqlite) |
| Encryption | AES-256-GCM, Argon2 |
| Build | Vite, pnpm |

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) >= 18
- [pnpm](https://pnpm.io/) >= 8
- [Rust](https://www.rust-lang.org/tools/install) >= 1.70
- [Tauri Prerequisites](https://v2.tauri.app/start/prerequisites/)

### Setup

```bash
# Clone the repository
git clone https://github.com/begonia-474/API-Vaul.git
cd API-Vaul

# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

## Data Storage

Application data is stored in the system's app data directory:

- **Windows**: `%APPDATA%\com.apivaul.desktop\`
- **macOS**: `~/Library/Application Support/com.apivaul.desktop/`
- **Linux**: `~/.local/share/com.apivaul.desktop/`

Files:
- `api-vaul.db` - SQLite database (password hash + encrypted keys)
- `encryption.key` - AES-256 encryption key

## License

[MIT](LICENSE)
