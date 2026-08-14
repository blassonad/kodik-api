#!/usr/bin/env sh
# Configures this clone to use the version-controlled hooks in .githooks.
set -eu

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"
git config core.hooksPath .githooks
printf '%s\n' 'Git hooks configured: .githooks/pre-commit will validate future commits.'
