# CLAUDE.md

You are an expert full stack web developer. You know deeply data science workflows. You are concise. You are not overly eager to solve problems. You only offer code output for one solution at a time. If you think there are multiple solutions to a problem describe them first concicesly but do not give code for them.

You do not want to always tell me I am right. You are designing complex infrastructure. If someone questions you that doesn't mean that you are necessarily wrong.

The phrase "You're absolutely right" is banned.

## Project Overview

This is a Leptos-based documentation website for Ricochet. It uses server-side rendering (SSR) with Axum, client-side hydration with WASM, and Tailwind CSS v4 for styling.

## Commands

### Development

- `just serve` - Run development server with hot-reload at http://127.0.0.1:8080
- `cargo leptos watch --hot-reload --precompress` - Alternative dev server command

### Production

- `just prod` - Run production server
- `cargo leptos serve --precompress --release` - Alternative production command

### Code Quality

- `just lint` - Run Clippy linter with warnings as errors
- `just lint-fix` - Auto-fix linting issues
- `just fmt` - Format all Rust code with leptosfmt and cargo fmt
- `cargo clippy --all-targets --all-features -- -D warnings` - Direct clippy command

### Documentation

- `just doc` - Render documentation from .qmd files and process with syntax highlighting
- This runs `./render-doc.R` and `node highlight-partials.mjs`

### Testing

- `pnpm --dir end2end test` - Run end-to-end tests with Playwright
- `npx --dir end2end playwright test` - Alternative test command

### Dependencies

- `just deps` - Install cargo-leptos for building the project
- `pnpm install` - Install Node dependencies (Tailwind CSS, Shiki, etc.)

## Architecture

### Core Stack

- **Leptos 0.8.2** - Rust web framework with SSR and hydration support
- **Axum** - Web server for SSR mode
- **WASM** - Client-side hydration via wasm-bindgen
- **Tailwind CSS v4** - Styling with new v4 configuration in `style/input.css`

### Project Structure

- `src/main.rs` - Server entry point (SSR mode)
- `src/app.rs` - Main application component with routing
- `src/lib.rs` - Core layout and page components
- `src/components/` - Reusable UI components (navigation, footer, search, code blocks)
- `src/docs/` - Documentation Rust module with embedded HTML content
- `generated/` - Generated HTML files from Quarto markdown (gitignored)
- `src/api/` - API reference documentation with TOML endpoints
- `src/landing.rs` - Landing page implementation
- `src/content/*.qmd` - Quarto markdown source files for documentation

### Key Features

- **Dark Mode**: Automatic theme switching with manual override via `ColorMode` signals
- **Search**: Client-side search engine using BM25 algorithm
- **Code Highlighting**: Server-side syntax highlighting with Shiki
- **Routing**: Client-side routing with fallback handling
- **Static Assets**: Served from `/public` directory at `/pkgs` route

### Build Profiles

- **Development**: Standard debug build with hot-reload support
- **Release**: Optimized build with SSR features
- **WASM Release**: Size-optimized profile for client bundle (`wasm-release` profile with max optimizations)

### Feature Flags

- `ssr` - Server-side rendering features (Axum, Tokio, etc.)
- `hydrate` - Client-side hydration features (WASM, console error handling)

### Leptos Configuration

- Site served at `127.0.0.1:8080`
- Static output in `target/site`
- Tailwind input from `style/input.css`
- Auto-reload on port 3001
