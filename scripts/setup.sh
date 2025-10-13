#!/bin/bash
# Setup script - Install dependencies and setup environment
# This mirrors the common setup steps from GitHub Actions

set -euo pipefail

# Change to the repository root directory (where this script's parent directory is located)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"


echo "🔧 Setting up development environment..."

# Check if Rust is installed
if ! command -v rustup &> /dev/null; then
    echo "❌ Rust/rustup not found. Please install Rust first:"
    echo "   Visit: https://rustup.rs/"
    echo "   Or run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Ensure we have the stable Rust toolchain
echo "📦 Installing/updating Rust stable toolchain..."
rustup toolchain install stable
rustup default stable

# Add WASM target
echo "📦 Adding wasm32v1-none target..."
rustup target add wasm32v1-none

# Note: Pre-commit checks are handled by GitHub Actions using pre-commit/action@v3.0.1
# Local pre-commit installation is optional for development convenience
echo "ℹ️  Pre-commit checks are handled by GitHub Actions"
echo "   You can optionally install pre-commit locally for development:"
echo "   - macOS: brew install pre-commit && pre-commit install"
echo "   - pip: pip install pre-commit && pre-commit install"

# Install Node.js dependencies if needed for host function audit
if command -v node &> /dev/null; then
    echo "✅ Node.js found for host function audit"
else
    echo "⚠️  Node.js not found. Host function audit will be skipped."
fi

echo "✅ Setup complete!"
echo ""
echo "Environment variables set:"
echo "  RUSTFLAGS=${RUSTFLAGS:-<not set>}"
echo ""
echo "Installed tools:"
echo "  - Rust toolchain: $(rustc --version)"
if command -v pre-commit &> /dev/null; then
    echo "  - Pre-commit: $(pre-commit --version) (optional)"
fi
if command -v node &> /dev/null; then
    echo "  - Node.js: $(node --version)"
fi
echo ""
echo "Next steps:"
echo "  - Run './scripts/run-all.sh' to run all tests"
echo "  - Run individual test scripts as needed"
echo ""
echo "💡 Tips:"
echo "   - Pre-commit checks run automatically in GitHub Actions"
echo "   - For local pre-commit hooks: install pre-commit and run 'pre-commit install'"
echo "   - If you encounter PATH issues, restart your shell or update your PATH"
