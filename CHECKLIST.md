# ✅ GitHub Actions Setup Checklist

## Before You Push

- [ ] Read `GITHUB_ACTIONS_SETUP.md` for full details
- [ ] Review `.github/workflows/README.md` for workflow documentation

## Enable GitHub Actions (Required)

### Step-by-Step Instructions:

1. **Navigate to your repository** on GitHub
2. **Click Settings** (under your repository name, gear icon)
3. **In the left sidebar**, click **Actions**, then click **General**
4. **Scroll down to "Workflow permissions"** section
5. **Select**: **"Read and write permissions"** (instead of "Read repository contents and packages permissions")
6. **Check the box**: **"Allow GitHub Actions to create and approve pull requests"**
7. **Click Save**

Visual location: Settings → Actions (left sidebar) → General → Workflow permissions (bottom of page)

## Push the Changes

```bash
git add .github/workflows/
git add GITHUB_ACTIONS_SETUP.md
git add dna/molecules/test-github-actions.sh
git add dna/atoms/secret-protection-help.sh
git add KNOWLEDGE_BASE/SECRET_PROTECTION_SETUP.md
git commit -m "feat: add automatic secret sanitization with GitHub Actions"
git push
```

## Test It Works

- [ ] Go to: https://github.com/nbiish/ainish-coder/actions
- [ ] Create a test file with a fake API key
- [ ] Push and watch the Actions tab
- [ ] Verify the secret was auto-cleaned

## Optional: Setup Local Protection Too

For the best security, also setup local protection:

```bash
bash dna/molecules/setup-secret-protection.sh
# Choose option 1: git-secrets
```

This gives you **three layers** of protection:
1. 🥇 git-secrets (local - blocks before commit)
2. 🥈 Pre-commit hook (local - blocks before push)
3. 🥉 GitHub Actions (cloud - auto-cleans after push)

## Files You Created

✅ `.github/workflows/auto-sanitize.yml` - Auto-cleans secrets
✅ `.github/workflows/detect-secrets.yml` - Detects and blocks secrets
✅ `.github/workflows/README.md` - Workflow documentation
✅ `.github/workflows/secret-scan.yml` - Legacy (disabled)
✅ `GITHUB_ACTIONS_SETUP.md` - Setup guide with visuals
✅ `dna/molecules/test-github-actions.sh` - Test workflows locally
✅ Updated `dna/atoms/secret-protection-help.sh` - Quick reference
✅ Updated `KNOWLEDGE_BASE/SECRET_PROTECTION_SETUP.md` - Full docs

## What Gets Auto-Cleaned

- Brave API keys (`BSA...`)
- Tavily API keys (`tvly-dev-...`)
- Local paths (`/Volumes/1tb-sandisk/...`)
- Memory file paths
- Generic API keys and secrets

## Quick Reference

Run anytime: `bash dna/atoms/secret-protection-help.sh`

## Troubleshooting

**Actions not running?**
- Check that workflow permissions are enabled (see above)
- Check the Actions tab for error messages

**Want to test locally first?**
```bash
brew install act
bash dna/molecules/test-github-actions.sh
```

**Need help?**
- Read `.github/workflows/README.md`
- Check `GITHUB_ACTIONS_SETUP.md`
- Run `bash dna/atoms/secret-protection-help.sh`

---

## 🎊 You're Ready!

Once you enable the permissions and push, GitHub Actions will automatically protect your secrets on every push!
