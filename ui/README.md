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

This script will automatically:

- Scan `examples/target/wasm32v1-none/release/` for all `.wasm` files
- Convert them to hex strings and embed them in the JavaScript
- Generate example buttons dynamically with proper title case names
- Update both the WASM data and UI buttons in `index.html`

No manual maintenance required! Just build your WASM files and run the script.

## Deployment

The UI is automatically deployed to GitHub Pages alongside the documentation at:
`https://ripple.github.io/xrpl-wasm-std/ui/`

## Architecture

- **Modular Structure**: HTML, CSS, and JavaScript properly separated
- **No Dependencies**: Uses CDN-hosted XRPL.js library
- **Dynamic WASM Loading**: Pre-built examples are automatically discovered and embedded
- **Responsive Design**: Works on desktop and mobile devices
- **Toast Notifications**: User-friendly feedback system
- **In-Memory State**: Account and escrow data persists during session only

## File Structure

- `index.html` - Main UI file
- `styles.css` - Stylesheet for UI components and layout
- `embed-wasm.sh` - Script to update embedded WASM examples
- `README.md` - This documentation

## Future Enhancements

### Wallet Integration

- Integrate with browser wallets (Crossmark, Gem Wallet, etc.)
- Support hardware wallets for secure signing
- Connect to mobile wallets

### Improved Testing

- Save and reload test configurations
- Transaction history and results tracking
- Pre-built test scenarios for common escrow types

### WASM Development

- Built-in code editor for writing Smart Escrow logic
- WASM validation and error checking
- Template library for common escrow patterns

### User Experience

- Better error messages and debugging info
- Export test results and configurations
