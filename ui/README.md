# Smart Escrow Testing UI

A web-based interface for testing XRPL Smart Escrows with embedded WASM examples.

## Features

- **Network Management**: Connect to local nodes or WASM Devnet
- **WASM Code Management**: Upload files, paste hex, or use pre-built examples
- **Account Management**: Generate and fund test accounts
- **Escrow Management**: Track and manage deployed escrows
- **Transaction Interface**: Deploy WASM as Smart Escrows and finish them
- **Testing Tools**: Quick test functionality and comprehensive logging

## Usage

### Local Development

1. Open `index.html` in your browser
2. Connect to a network (local node or WASM Devnet)
3. Generate test accounts
4. Load WASM code (examples are embedded)
5. Deploy Smart Escrows and test them

### Updating WASM Examples

To embed the latest built WASM files:

```bash
# From the repository root
./ui/embed-wasm.sh
```

This script will:

- Read WASM files from `examples/target/wasm32v1-none/release/`
- Convert them to hex strings
- Embed them directly in the UI

## Deployment

The UI is automatically deployed to GitHub Pages alongside the documentation at:
`https://your-repo.github.io/ui/`

## Architecture

- **Single File**: Self-contained HTML with embedded CSS and JavaScript
- **No Dependencies**: Uses CDN-hosted XRPL.js library
- **Embedded WASM**: Pre-built examples are hex-encoded and embedded
- **Local Storage**: Account and escrow data persists in memory only

## File Structure

- `index.html` - Main UI file
- `embed-wasm.sh` - Script to update embedded WASM examples
- `README.md` - This documentation
