#!/bin/bash
# Script to embed WASM files as hex strings in the UI

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

UI_FILE="ui/index.html"
WASM_DIR="examples/target/wasm32v1-none/release"

echo "🔧 Embedding WASM files into UI..."

# Function to convert WASM file to hex string
wasm_to_hex() {
    local wasm_file="$1"
    if [[ -f "$wasm_file" ]]; then
        xxd -p "$wasm_file" | tr -d '\n'
    else
        echo "null"
    fi
}

# Read the current UI file
cp "$UI_FILE" "$UI_FILE.backup"

# Generate the WASM hex strings
echo "📦 Converting WASM files to hex..."

LEDGER_SQN_HEX=$(wasm_to_hex "$WASM_DIR/ledger_sqn.wasm")
NOTARY_HEX=$(wasm_to_hex "$WASM_DIR/notary.wasm")
KYC_HEX=$(wasm_to_hex "$WASM_DIR/kyc.wasm")
NFT_OWNER_HEX=$(wasm_to_hex "$WASM_DIR/nft_owner.wasm")
ORACLE_HEX=$(wasm_to_hex "$WASM_DIR/oracle.wasm")

echo "📝 Updating UI file..."

# Create the replacement for the EMBEDDED_WASM object
cat > /tmp/embedded_wasm.js << EOF
        // Embedded WASM examples as hex strings
        // To update these, run: ./scripts/embed-wasm.sh
        const EMBEDDED_WASM = {
            ledger_sqn: "${LEDGER_SQN_HEX}",
            notary: "${NOTARY_HEX}",
            kyc: "${KYC_HEX}",
            nft_owner: "${NFT_OWNER_HEX}",
            oracle: "${ORACLE_HEX}"
        };
EOF

# Replace the EMBEDDED_WASM section in the UI file
awk '
BEGIN { in_embedded_wasm = 0; }
/^        \/\/ Embedded WASM examples as hex strings/ {
    in_embedded_wasm = 1;
    # Read and print the replacement
    while ((getline line < "/tmp/embedded_wasm.js") > 0) {
        print line;
    }
    close("/tmp/embedded_wasm.js");
    next;
}
/^        };$/ && in_embedded_wasm {
    in_embedded_wasm = 0;
    next;
}
!in_embedded_wasm { print; }
' "$UI_FILE.backup" > "$UI_FILE"

# Clean up
rm -f /tmp/embedded_wasm.js "$UI_FILE.backup"

echo "✅ WASM files embedded successfully!"
echo ""
echo "📊 Embedded files:"
[[ -f "$WASM_DIR/ledger_sqn.wasm" ]] && echo "  ✓ ledger_sqn ($(stat -f%z "$WASM_DIR/ledger_sqn.wasm" 2>/dev/null || stat -c%s "$WASM_DIR/ledger_sqn.wasm" 2>/dev/null || echo "?") bytes)"
[[ -f "$WASM_DIR/notary.wasm" ]] && echo "  ✓ notary ($(stat -f%z "$WASM_DIR/notary.wasm" 2>/dev/null || stat -c%s "$WASM_DIR/notary.wasm" 2>/dev/null || echo "?") bytes)"
[[ -f "$WASM_DIR/kyc.wasm" ]] && echo "  ✓ kyc ($(stat -f%z "$WASM_DIR/kyc.wasm" 2>/dev/null || stat -c%s "$WASM_DIR/kyc.wasm" 2>/dev/null || echo "?") bytes)"
[[ -f "$WASM_DIR/nft_owner.wasm" ]] && echo "  ✓ nft_owner ($(stat -f%z "$WASM_DIR/nft_owner.wasm" 2>/dev/null || stat -c%s "$WASM_DIR/nft_owner.wasm" 2>/dev/null || echo "?") bytes)"
[[ -f "$WASM_DIR/oracle.wasm" ]] && echo "  ✓ oracle ($(stat -f%z "$WASM_DIR/oracle.wasm" 2>/dev/null || stat -c%s "$WASM_DIR/oracle.wasm" 2>/dev/null || echo "?") bytes)"

echo ""
echo "🎯 Ready to use! Open ui/index.html and try the Examples tab."
