## Preconditions
- Verify repository is initialized and you are in the correct project directory.
- Confirm current branch and remote:
  - `git status`
  - `git branch --show-current`
  - `git remote -v`

## Stage Changes
- Stage all intended modifications:
  - `git add -A`
  - Optional: review staged set with `git diff --staged`

## Commit
- Create a descriptive commit using the provided message:
  - `git commit -m "Updated commit messages to be very descriptive."`
- If pre-commit hooks exist, ensure they pass; fix any issues and re-run the commit.

## Push to Main
- Ensure you are on `main` (switch if needed):
  - `git checkout main` (create with `git checkout -b main` if it doesn't exist)
- Push to remote:
  - `git push origin main`
- If upstream is not set:
  - `git push -u origin main`

## Validation
- Confirm remote state:
  - `git log -n 1 --oneline` and `git ls-remote --heads origin`
- If CI/CD is configured, monitor pipeline status.

## Handling Common Issues
- Authentication failures: set credentials or SSH keys and retry.
- Diverged branch: pull and rebase (`git fetch origin && git rebase origin/main`) then push.
- Large files/LFS: ensure `.gitattributes` and LFS tracking are correct before pushing.

## Next Step
- Once approved, I will execute the exact `git commit` and `git push origin main` commands and report the results.