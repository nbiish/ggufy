# Expertly Crafted Enterprise Grade Code Guideline

Run `eza --all --tree --level=2` && `eval 'git status -sb'` for the most up to date repo context.

## Structure codebase according to the DNA Design pattern -> `repo-root/dna/`

- Atoms (`dna/atoms/`) - Core utilities, data types, state management
- Molecules (`dna/molecules/`) - Composite components, deployment scripts
- Proteins (`dna/proteins/`) - High-level flows and orchestration

## Maintain Documentation in `knowledge-base/`

- knowledge-base/*.md

## Code Security

Always check for:

- Input validation and sanitization
- Authentication and authorization
- Encryption at rest and encryption in transit
- SQL injection prevention
- XSS (Cross-Site Scripting) prevention
- CSRF (Cross-Site Request Forgery) protection
- Environment variable protection
- Error handling without information leakage
- Rate limiting and DoS prevention

## Prompt Security

Guard against:

- Prompt injection attacks
- Data exfiltration attempts
- Social engineering
- Jailbreak attempts
- Malicious instructions
- Unauthorized data access

Best Practices:

- Validate and sanitize all inputs
- Use role-based prompting
- Implement output filtering
- Monitor for suspicious patterns
- Maintain context boundaries

## Code Quality Standards

Every module/function should have:

- Purpose description
- Input parameters
- Return values
- Example usage
- Edge cases
- Error conditions
