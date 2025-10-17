# KYC Credential Smart Escrow

A compliance-focused smart escrow for the XRPL that verifies credential-based authorization before releasing funds.

## Overview

This smart escrow demonstrates how to implement Know Your Customer (KYC) compliance using XRPL's credential system. It verifies that the escrow destination account has a valid "termsandconditions" credential before allowing funds to be released.

- Verifies credential existence before release
- Demonstrates credential keylet generation
- Shows compliance-focused escrow patterns
- Implements fail-safe authorization logic

The Rust code demonstrates how to integrate XRPL's credential system with smart escrow conditional logic for regulatory compliance scenarios.

## Functionality

### Core Components

- **Credential Verification**: Checks for specific credential type on destination account
- **Keylet Generation**: Creates credential keylets for ledger object lookup
- **Authorization Logic**: Only releases funds when credentials are present
- **Error Handling**: Graceful failure when credentials are missing

### Key Functions

- `finish()`: Main entry point that verifies credential and returns authorization decision
- **Credential Lookup**: Uses credential keylet to check ledger state
- **Safe Defaults**: Returns 0 (deny) when credentials are absent or errors occur

## Code Structure

```rust
#![no_std]
#![no_main]

use xrpl_wasm_std::core::current_tx::escrow_finish::EscrowFinish;
use xrpl_wasm_std::core::types::keylets::credential_keylet;
use xrpl_wasm_std::host::cache_ledger_obj;

const ISSUER: [u8; 20] = /* Credential issuer account */;
const CREDENTIAL_TYPE: &[u8] = b"termsandconditions";

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    let tx = EscrowFinish;

    // Get the destination account
    let destination = match tx.get_destination() {
        Ok(dest) => dest,
        Err(_) => return 0, // No destination, deny
    };

    // Generate credential keylet
    let keylet = match credential_keylet(&destination, &ISSUER, CREDENTIAL_TYPE) {
        Ok(k) => k,
        Err(_) => return 0, // Keylet error, deny
    };

    // Check if credential exists in ledger
    match cache_ledger_obj(&keylet.data) {
        Ok(_) => 1,   // Credential exists, allow
        Err(_) => 0,  // No credential, deny
    }
}
```

## Building

### Prerequisites

- Rust with `wasm32v1-none` target
  - This is necessary for blockchain deployments because WebAssembly does not require a specific vendor (e.g., `apple`) or operating system (e.g., `darwin`), so both are `unknown`
- XRPL standard library (dependency)
- Node.js 18+ (for testing)

### Build Commands

```bash
# Debug build
cargo build --target wasm32v1-none

# Release build (optimized)
cargo build --target wasm32v1-none --release
```

The compiled WASM will be at:

```
target/wasm32v1-none/release/kyc.wasm
```

## Testing

### Integration Test Suite

The comprehensive integration test demonstrates the full KYC workflow:

```bash
# From the repository root
./scripts/run-tests.sh examples/smart-escrows/kyc

# Or with explicit CI mode
CI=1 ./scripts/run-tests.sh examples/smart-escrows/kyc
```

### Test Workflow

The integration test (`run_test.js`) performs a realistic KYC compliance scenario:

1. **Connect** to WASM Devnet
2. **Create accounts** - sender, receiver, and credential issuer
3. **Deploy escrow** with KYC WASM as finish function
4. **First finish attempt** - Should fail with `tecWASM_REJECTED` (no credential)
5. **Create credential** - Issue "termsandconditions" credential to receiver
6. **Second finish attempt** - Should succeed with `tesSUCCESS` (credential exists)
7. **Verify results** - Funds transferred and escrow completed

### Expected Results

| Phase               | Transaction        | Expected Result    | Reason               |
| ------------------- | ------------------ | ------------------ | -------------------- |
| Initial Finish      | `EscrowFinish`     | `tecWASM_REJECTED` | No credential exists |
| Credential Creation | `CredentialCreate` | `tesSUCCESS`       | Credential issued    |
| Final Finish        | `EscrowFinish`     | `tesSUCCESS`       | Credential verified  |

### Test Scenarios

The test covers:

#### Positive Cases

- Successful credential verification
- Proper fund release after compliance
- Correct transaction result codes

#### Negative Cases

- Escrow rejection without credentials
- Error handling for missing accounts
- Graceful failure for invalid keylets

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        XRP Ledger                               │
│                                                                 │
│  ┌─────────────────┐              ┌─────────────────────────┐   │
│  │   Credential    │              │    KYC Smart Escrow     │   │
│  │   Object        │              │    (WASM Module)        │   │
│  │                 │              │                         │   │
│  │ ┌─────────────┐ │              │ finish() function:      │   │
│  │ │ Type:       │ │◄─────────────┤                         │   │
│  │ │ "terms..."  │ │              │ 1. Get destination      │   │
│  │ │ Subject:    │ │              │                         │   │
│  │ │ destination │ │              │ 2. Generate keylet      │   │
│  │ │ Issuer:     │ │              │                         │   │
│  │ │ issuer_acc  │ │              │ 3. Check credential     │   │
│  │ └─────────────┘ │              │                         │   │
│  └─────────────────┘              │ 4. Return decision      │   │
│                                   │    (1=allow, 0=deny)    │   │
│                                   └─────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

## Credential System

### Credential Structure

XRPL credentials consist of:

- **Subject**: The account that holds the credential (destination)
- **Issuer**: The account that issued the credential (trusted authority)
- **Type**: The credential type identifier (e.g., "termsandconditions")

### Keylet Generation

```rust
// Generate a unique identifier for the credential ledger object
let keylet = credential_keylet(&subject, &issuer, credential_type)?;

// Use keylet to check if credential exists
let slot = cache_ledger_obj(&keylet.data)?;
```

### Compliance Workflow

1. **User Registration**: User completes KYC process with issuing authority
2. **Credential Issuance**: Authority creates credential on XRPL for user
3. **Escrow Creation**: Escrow created with KYC verification requirement
4. **Funds Release**: Smart escrow verifies credential before releasing funds

## Use Cases

### Regulatory Compliance

- **Financial Services**: Verify customer identity before fund access
- **Securities Trading**: Ensure accredited investor status
- **Cross-Border Payments**: Comply with AML/KYC requirements
- **Digital Asset Custody**: Verify account ownership and compliance

### Business Applications

- **Terms Acceptance**: Verify users have accepted terms and conditions
- **License Verification**: Check professional licensing credentials
- **Age Verification**: Confirm user age for restricted services
- **Identity Confirmation**: Validate user identity for high-value transactions

## Configuration

### Credential Parameters

The contract is configured with:

```rust
// The account authorized to issue credentials
const ISSUER: [u8; 20] = /* Replace with your issuer account */;

// The type of credential required
const CREDENTIAL_TYPE: &[u8] = b"termsandconditions";
```

### Customization Options

- **Multiple Issuers**: Check credentials from different authorities
- **Credential Types**: Support various compliance requirements
- **Expiration Checks**: Verify credential validity periods
- **Threshold Logic**: Require multiple credentials

## Security Considerations

### Trust Model

- **Issuer Trust**: Contract trusts the specified issuer account
- **Credential Integrity**: Relies on XRPL's credential system security
- **Keylet Security**: Uses cryptographically secure keylet generation

### Best Practices

- **Validate Inputs**: Always check account and credential data
- **Fail Safely**: Return 0 (deny) on any error or uncertainty
- **Minimize Trust**: Limit number of trusted credential issuers
- **Monitor Usage**: Log credential checks for audit trails

### Potential Risks

- **Issuer Compromise**: Malicious issuer could create invalid credentials
- **Credential Revocation**: System doesn't check for revoked credentials
- **Key Management**: Issuer account security is critical

## Future Enhancements

- [ ] Support for credential expiration dates
- [ ] Multiple credential type requirements
- [ ] Credential revocation checking
- [ ] Dynamic issuer configuration
- [ ] Audit logging and compliance reporting
- [ ] Integration with external KYC providers

## Troubleshooting

### Common Issues

| Issue                       | Symptoms            | Solution                                   |
| --------------------------- | ------------------- | ------------------------------------------ |
| **Credential not found**    | `tecWASM_REJECTED`  | Ensure credential exists with correct type |
| **Wrong issuer**            | Always fails        | Verify issuer account in contract config   |
| **Keylet generation error** | Contract returns 0  | Check account format and credential type   |
| **Cache full**              | `NoFreeSlots` error | Optimize contract to use fewer cache slots |

### Debugging Tips

```rust
// Add debug traces to understand execution flow
use xrpl_wasm_std::host::trace::{trace, trace_data, DataRepr};

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    trace("KYC: Starting credential check")?;

    let destination = match tx.get_destination() {
        Ok(dest) => {
            trace_data("KYC: Destination", &dest, DataRepr::AsHex)?;
            dest
        },
        Err(_) => {
            trace("KYC: Error getting destination")?;
            return 0;
        }
    };

    // More debugging...
}
```

## Next Steps

After mastering this example, explore:

1. **[Oracle Example](../oracle/)** - Price-based conditional logic
2. **[Notary Example](../notary/)** - Multi-signature authorization
3. **[NFT Owner Example](../nft_owner/)** - Asset ownership verification
4. **[API Reference](../../../docs/api-reference.md)** - Complete credential APIs

## References

- **[XRPL Credentials](https://xrpl.org/docs/concepts/accounts/decentralized-identifiers)** - Credential system documentation
- **[Smart Escrows XLS](https://github.com/XRPLF/XRPL-Standards/discussions/270)** - Smart escrow specification
- **[KYC Compliance](https://en.wikipedia.org/wiki/Know_your_customer)** - Know Your Customer overview
- **[Getting Started Guide](../../../docs/getting-started.md)** - Complete setup instructions
