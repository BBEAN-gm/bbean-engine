# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | Yes       |

## Reporting a Vulnerability

If you discover a security vulnerability in BBEAN Engine, please report it responsibly.

**Do not open a public issue for security vulnerabilities.**

Instead, please send an email to the maintainers with:

1. A description of the vulnerability
2. Steps to reproduce the issue
3. Potential impact assessment
4. Suggested fix (if any)

We will acknowledge receipt within 48 hours and provide a detailed response within 7 days.

## Security Considerations

### Proof Validation

The Proof-of-Brew system relies on SHA-256 hash difficulty. The default difficulty of 16 bits provides a balance between security and usability. For production deployments, consider adjusting based on your threat model.

### Network Transport

All node communication should be encrypted in transit. The WebSocket transport layer supports TLS when configured with appropriate certificates.

### Solana Program

The on-chain program includes arithmetic overflow protection and authority validation. All token operations use checked arithmetic to prevent overflow attacks.

### Configuration

- Never commit configuration files containing private keys
- Use environment variables for sensitive values
- Restrict RPC endpoint access in production
