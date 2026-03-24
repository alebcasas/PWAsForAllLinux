# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

We take the security of PWAsForAllLinux seriously. If you discover a security vulnerability, please report it responsibly.

### How to Report

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please report them via:

1. **GitHub Security Advisories** (Preferred)
   - Go to [Security Advisories](https://github.com/pwasforalllinux/pwasforalllinux/security/advisories/new)
   - Create a new security advisory

2. **Email**
   - Send details to: security@pwasforalllinux.org
   - Include "Security Vulnerability" in the subject

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Affected versions
- Potential impact
- Suggested fix (if available)

### Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Resolution**: Depends on severity, typically within 30 days

### Disclosure Policy

- We follow [Responsible Disclosure](https://en.wikipedia.org/wiki/Responsible_disclosure)
- Security fixes are released as patch versions
- CVEs will be assigned for significant vulnerabilities
- Credit will be given to reporters (unless anonymity is requested)

## Security Best Practices

When using PWAsForAllLinux:

1. **Only install PWAs from trusted websites**
2. **Keep the application updated**
3. **Review permissions requested by websites**
4. **Use isolated profiles for sensitive applications**

## Known Security Considerations

- Each PWA runs in an isolated profile
- Web content is rendered by WebKitGTK
- No special system privileges are required
- All data is stored locally in user directories

Thank you for helping keep PWAsForAllLinux secure! 🔒
