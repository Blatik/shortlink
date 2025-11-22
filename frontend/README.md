# Rust Frontend (Leptos + WASM)

This is the Rust/WebAssembly frontend for the URL Shortener, built with [Leptos](https://leptos.dev/).

## Quick Start

### Development Server

```bash
trunk serve
```

Visit `http://localhost:8080`

### Production Build

```bash
trunk build --release
```

Output goes to `../docs` (configured in `Trunk.toml`)

## Project Structure

```
src/
├── main.rs              # Entry point
├── app.rs               # Router setup
├── api.rs               # API client
├── auth.rs              # Google Auth
├── components/
│   ├── shortener.rs     # URL form
│   ├── dashboard.rs     # Links table
│   └── charts.rs        # Bar charts
└── pages/
    ├── home.rs          # Landing page
    └── analytics.rs     # Analytics page
```

## Configuration

### API URL

Edit `src/api.rs`:

```rust
const API_BASE: &str = "https://your-worker.workers.dev";
```

### Google Client ID

Edit `index.html` and `src/pages/home.rs`:

```html
data-client_id="YOUR_GOOGLE_CLIENT_ID_HERE"
```

## Tech Stack

- **Leptos 0.6** - Reactive UI framework
- **gloo-net** - HTTP client for WASM
- **leptos_router** - Client-side routing
- **wasm-bindgen** - JS interop
- **Trunk** - WASM bundler

## Features

✅ URL shortening with custom aliases  
✅ User dashboard with reactive updates  
✅ Analytics with custom SVG charts  
✅ Google Sign-In integration  
✅ Copy to clipboard  
✅ Type-safe API calls  
✅ Client-side routing
