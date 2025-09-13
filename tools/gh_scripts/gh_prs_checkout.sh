#!/nix/store/5z7w1hywa0i56d4a90fs77dj6im21jkf-bash-5.3p3/bin/bash
# Script to checkout a GitHub Pull Request locally
if [ -z "$1" ]; then
  echo "Usage: $0 <PR_NUMBER>"
  exit 1
fi
gh pr checkout "$@"