#!/usr/bin/env bash
# verify-rebrand.sh — fail CI if forbidden legacy brand tokens leak back into the workspace.
# Canonical brand must be "KayStichs". Anything else is a regression.
set -euo pipefail

ROOT_DIR="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
cd "$ROOT_DIR"

# Tokens that must NEVER appear outside the documented allow-list.
FORBIDDEN=(
  "Learnault"
  "learnault"
  "LEARNAULT"
)

# Files where historical references are intentional and must be allowed.
ALLOWLIST_REGEX='^(BRANDING\.md|CHANGELOG\.md|course_completion_payout_audit\.md|FINAL_CI_REPORT\.md)$'

violations=0
echo "[rebrand] searching for forbidden tokens..."

rg_or_grep() {
  if command -v rg >/dev/null 2>&1; then
    rg --hidden --no-ignore -n "$@"
  else
    grep -RInE "$@"
  fi
}

for tok in "${FORBIDDEN[@]}"; do
  # shellcheck disable=SC2086
  matches=$(rg_or_grep "${tok}" || true)
  if [[ -z "$matches" ]]; then
    continue
  fi
  while IFS= read -r line; do
    file=$(echo "$line" | cut -d: -f1)
    case "$file" in
      $ALLOWLIST_REGEX) ;;
      *)
        echo "[rebrand] FORBIDDEN token '${tok}' found in ${line}"
        violations=$((violations + 1))
        ;;
    esac
  done <<<"$matches"
done

# Positive check: ensure the canonical brand token is present somewhere on disk.
if ! rg_or_grep "KayStichs" >/dev/null 2>&1; then
  echo "[rebrand] canonical brand token 'KayStichs' was NOT found anywhere"
  violations=$((violations + 1))
fi

if [[ "$violations" -gt 0 ]]; then
  echo "[rebrand] FAIL: ${violations} forbidden or missing brand reference(s)"
  exit 1
fi

echo "[rebrand] OK: brand tokens are clean"
