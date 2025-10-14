# XRPL WASM Tests

This directory contains end-to-end integration tests for XRPL WASM smart contracts.

## Quick Start

### Running Tests Against Devnet (Default)

By default, all tests run against the public WASM devnet:

```bash
# From repository root
./scripts/e2e-tests.sh
```

This uses the devnet URL: `wss://wasm.devnet.rippletest.net:51233`

### Running Tests Against Local rippled

To test against a local rippled instance, set the `RIPPLED_URL` environment variable:

```bash
# Start rippled in standalone mode
rippled --standalone

# Run tests against it
RIPPLED_URL="ws://127.0.0.1:6006" ./scripts/e2e-tests.sh
```

### Docker rippled

If you're running rippled in Docker, make sure to expose the WebSocket port:

```bash
# Example Docker command
docker run -p 6006:6006 ripple/rippled:latest --standalone

# Run tests
RIPPLED_URL="ws://127.0.0.1:6006" ./scripts/e2e-tests.sh
```

## Test Scripts

### `setup_ledger.js`

Sets up test accounts and saves them to `wallets.json`.

**Usage:**

```bash
# Use default (devnet)
node setup_ledger.js

# Use custom URL
node setup_ledger.js "ws://127.0.0.1:6006"

# Or via environment variable
RIPPLED_URL="ws://127.0.0.1:6006" node setup_ledger.js
```

**What it does:**

- Connects to the specified rippled server
- Creates and funds 5 test wallets
- Saves wallet credentials to `wallets.json`
- For localhost: Uses the genesis account to fund wallets
- For devnet: Uses the faucet to fund wallets

### `run_single_test.js`

Runs a single integration test for a WASM contract.

**Usage:**

```bash
# Use default (devnet)
node run_single_test.js <test_dir> <wasm_file>

# Use custom URL
node run_single_test.js <test_dir> <wasm_file> "ws://127.0.0.1:6006"

# Or via environment variable
RIPPLED_URL="ws://127.0.0.1:6006" node run_single_test.js <test_dir> <wasm_file>
```

### `deploy_wasm_code.js`

Helper module for deploying WASM contracts to the ledger.

## Environment Variables

| Variable      | Default                                  | Description                          |
|---------------|------------------------------------------|--------------------------------------|
| `RIPPLED_URL` | `wss://wasm.devnet.rippletest.net:51233` | The rippled server URL to connect to |

## Troubleshooting

### Connection Refused

If you get a connection error:

```
Error: connect ECONNREFUSED 127.0.0.1:6006
```

**Solutions:**

1. Make sure rippled is running: `rippled server_info`
2. Check that the WebSocket port is open: `netstat -an | grep 6006`
3. Verify your rippled configuration file has WebSocket enabled
4. Try the default devnet by unsetting `RIPPLED_URL`

### Wrong Port

If you're not sure which port your rippled is using:

```bash
# Check rippled configuration
rippled server_info | grep -i port

# Or check the config file (location varies by installation)
cat ~/.config/ripple/rippled.cfg | grep -A 5 "\[port_ws_public\]"
```

### Genesis Account Not Found (Localhost Only)

If you see errors about the genesis account when testing locally:

```
Account not found: rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh
```

This means you're not running in standalone mode. Either:

1. Start rippled with `--standalone` flag
2. Use the devnet instead (unset `RIPPLED_URL`)
