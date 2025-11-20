# MCP Configuration Setup

## Security Notice

⚠️ **Never commit your actual API keys or local paths to git!**

## Setup Instructions

### 1. Copy Template Files

```bash
# For each tier, copy the template
cp CONFIGURATIONS/MCP/settings.json.template CONFIGURATIONS/MCP/TIER_1/settings.json
cp CONFIGURATIONS/MCP/settings.json.template CONFIGURATIONS/MCP/TIER_2/settings.json
cp CONFIGURATIONS/MCP/settings.json.template CONFIGURATIONS/MCP/TIER_3/settings.json
```

### 2. Replace Placeholders

Edit each `settings.json` file and replace:

- `YOUR_BRAVE_API_KEY_HERE` → Your actual Brave API key
- `YOUR_TAVILY_API_KEY_HERE` → Your actual Tavily API key
- `/path/to/your/mcp/servers/*` → Your actual MCP server paths
- `/path/to/your/memory/memories.jsonl` → Your actual memory file path

### 3. Setup Secret Protection (Choose One)

#### Option A: git-secrets (Recommended - Local Protection)

```bash
# Run the setup script
chmod +x .git-secrets-setup.sh
./.git-secrets-setup.sh
```

This will:
- Install git-secrets via Homebrew
- Configure patterns to catch API keys and paths
- Install pre-commit hooks to prevent secrets from being committed

#### Option B: Manual Sanitization Script

```bash
# Before committing, run:
bash population/atoms/sanitize-settings.sh

# Review changes
git diff

# Commit
git add CONFIGURATIONS/
git commit -m "Update configurations"
```

#### Option C: GitHub Actions (Cloud Protection)

Already configured in `.github/workflows/secret-scan.yml`
- Automatically scans on push/PR
- Uses TruffleHog and Gitleaks
- Blocks merges if secrets detected

## Environment Variables (Alternative)

Instead of hardcoding, use environment variables:

```json
{
  "env": {
    "BRAVE_API_KEY": "${BRAVE_API_KEY}"
  }
}
```

Then set in your shell:
```bash
export BRAVE_API_KEY="your-key-here"
```

## What's Protected

- Brave API keys (pattern: `BSA[a-zA-Z0-9]{27}`)
- Tavily API keys (pattern: `tvly-*`)
- Local file paths (`/Volumes/1tb-sandisk/`)
- Generic API key patterns
- Passwords and secrets

## Testing

```bash
# Test if secrets would be caught
git secrets --scan CONFIGURATIONS/MCP/*/settings.json

# Scan entire history
git secrets --scan-history
```

## Troubleshooting

If git-secrets blocks a legitimate commit:
```bash
# Temporarily allow (use with caution!)
git commit --no-verify -m "message"

# Or remove the pattern
git secrets --remove-pattern 'pattern-to-remove'
```
