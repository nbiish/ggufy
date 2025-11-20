# Security Implementation Guide

> **TOON Format** (Token-Oriented Object Notation) - Optimized for LLM comprehension and implementation

```toon
@Security_Architecture {
  Strategy: "Defense-in-Depth + Post-Quantum Readiness";
  
  Tools: [
    { 
      Name: "CodeQL", 
      Type: "SAST", 
      Trigger: "Push/PR",
      Output: "SARIF -> Copilot Autofix"
    },
    { 
      Name: "OWASP ZAP", 
      Type: "DAST", 
      Trigger: "Weekly/Manual",
      Config: "Full Scan + Ajax Spider"
    },
    { 
      Name: "CBOM Kit", 
      Type: "PQC Audit", 
      Trigger: "Weekly",
      Goal: "Inventory + Hybrid Mode Enforcement"
    }
  ];

  Workflow_Integration: {
    Reporting: "GitHub Security Tab (SARIF)";
    Blocking: "High Severity = Block Merge";
    Remediation: "Copilot Autofix";
  };
}

@OWASP_ZAP_Config {
  Action: "zaproxy/action-full-scan";
  Target: "${{ inputs.target }} || localhost:3000";
  Auth: "Bearer Token via Env Vars";
  
  Remediation_Steps: [
    "1. View Alert in Security Tab",
    "2. Identify Vulnerable Endpoint",
    "3. Prompt Copilot: 'Fix ZAP Alert [Name] in [File]'",
    "4. Verify with Local Scan"
  ];
}

@PQC_Strategy {
  Standard: "NIST FIPS 203/204/205";
  Mode: "Hybrid (Classical + PQC)";
  
  Patterns: [
    { Use: "Key Encapsulation", Algo: "X25519 + ML-KEM" },
    { Use: "Signatures",        Algo: "Ed25519 + ML-DSA" },
    { Use: "Hashing",           Algo: "SHA-3 / SHAKE"    }
  ];

  Copilot_Prompts: {
    Inventory: "Analyze codebase for cryptographic algorithms.",
    Migration: "Refactor RSA-2048 to use Hybrid ML-KEM.",
    Verify: "Check if this crypto implementation is quantum-safe."
  };
}
```
