# Enterprise Grade Code Guidelines

> **TOON Format** (Token-Oriented Object Notation) - Compact, human-readable encoding optimized for LLM prompts.
> Specification: [TOON v2.0](https://github.com/toon-format/toon)

```toon
project_setup[2]{type,requirement}:
  python,uv venv .venv
  structure_view,eza --all --tree --level=2 --git
dna_architecture:
  base_path: repo-root/dna/
  layers[3]{name,path,purpose}:
    Atoms,dna/atoms/,"Core utilities, data types, state management"
    Molecules,dna/molecules/,"Composite components, deployment scripts"
    Proteins,dna/proteins/,High-level flows and orchestration
knowledge_base:
  path: knowledge-base/
  required: true
security_checks[9]{category,requirement}:
  input,Validation and sanitization
  auth,Authentication and authorization
  encryption,"At rest, in transit, use asymmetric (RSA, Ed25519)"
  sql,Injection prevention
  xss,Cross-Site Scripting prevention
  csrf,Cross-Site Request Forgery protection
  env,Environment variable protection
  errors,Handling without information leakage
  dos,Rate limiting and DoS prevention
post_quantum_cryptography:
  approach: Hybrid mode (classical + PQC) for defense-in-depth
  nist_standards[4]{algorithm,fips,use_case}:
    ML-KEM,FIPS-203,Key encapsulation
    ML-DSA,FIPS-204,Digital signatures
    SLH-DSA,FIPS-205,Hash-based signatures
    HQC,Draft-2026,Backup KEM
  implementation: "Use PQC alongside classical algorithms, enable crypto-agility"
threat_protection[2]{threat_type,mitigation_required}:
  Data exfiltration,Access controls & monitoring
  Unauthorized access,Authentication & authorization
best_practices[5]{practice,implementation}:
  Input handling,Validate and sanitize all inputs
  Prompting,Use role-based prompting
  Output,Implement output filtering
  Monitoring,Monitor for suspicious patterns
  Context,Maintain context boundaries
module_documentation[6]{component,required}:
  Purpose,Description of functionality
  Inputs,Parameter specifications
  Outputs,Return value types
  Examples,Usage demonstrations
  Edge cases,Boundary conditions
  Errors,Error conditions & handling
```
