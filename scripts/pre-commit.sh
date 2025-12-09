#!/bin/bash

# Run pre-commit hooks if available
echo "=================================================="
echo "🔧 Running pre-commit hooks..."
echo "=================================================="
if command -v pre-commit &> /dev/null; then
    if pre-commit run --all-files; then
        echo "✅ pre-commit hooks completed successfully"
    else
        echo "❌ pre-commit hooks failed"
        exit 1
    fi
else
    echo "ℹ️  pre-commit not installed, skipping (install with: pip install pre-commit)"
fi
echo ""
