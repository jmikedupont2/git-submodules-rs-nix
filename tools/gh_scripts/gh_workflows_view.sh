#!/nix/store/5z7w1hywa0i56d4a90fs77dj6im21jkf-bash-5.3p3/bin/bash
# Script to view details of a GitHub Actions workflow run
if [ -z "$1" ]; then
  echo "Usage: $0 <RUN_ID> [flags]"
  exit 1
fi
gh run view "$@"