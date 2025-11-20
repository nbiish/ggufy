#!/bin/bash
# Setup git-secrets to prevent committing API keys and sensitive paths

set -e

echo "🔒 Setting up git-secrets..."

# Install git-secrets (macOS)
if ! command -v git-secrets &> /dev/null; then
    echo "Installing git-secrets via Homebrew..."
    brew install git-secrets
fi

# Initialize git-secrets in this repo
git secrets --install
git secrets --register-aws

# Add custom patterns for your specific secrets
echo "Adding custom secret patterns..."

# API Key patterns
git secrets --add 'BSA[a-zA-Z0-9]{27}'  # Brave API Key pattern
git secrets --add 'tvly-[a-zA-Z0-9-]{30,}'  # Tavily API Key pattern
git secrets --add '(BRAVE_API_KEY|tavilyApiKey)[\s]*[:=][\s]*["\047][^"\047\s]{10,}["\047]'

# Path patterns (your specific paths)
git secrets --add '/Volumes/1tb-sandisk/'
git secrets --add 'MEMORY_FILE_PATH.*\.jsonl'

# Generic API key patterns
git secrets --add '[aA][pP][iI]_?[kK][eE][yY].*['\''"][0-9a-zA-Z]{32,}['\''"]'
git secrets --add '[sS][eE][cC][rR][eE][tT].*['\''"][0-9a-zA-Z]{32,}['\''"]'
git secrets --add '[pP][aA][sS][sS][wW][oO][rR][dD].*['\''"][^\s]{8,}['\''"]'

echo "✅ git-secrets configured!"
echo ""
echo "🧪 Testing on your files..."
git secrets --scan CONFIGURATIONS/MCP/*/settings.json || true
echo ""
echo "💡 To scan all files: git secrets --scan"
echo "💡 To scan history: git secrets --scan-history"
