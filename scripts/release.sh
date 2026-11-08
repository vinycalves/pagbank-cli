#!/usr/bin/env bash
set -euo pipefail

if [ $# -ne 1 ]; then
  echo "Uso: $0 <versão>"
  echo "Exemplo: $0 0.2.0"
  exit 1
fi

VERSION="$1"
BRANCH=$(git branch --show-current)

if [ "$BRANCH" != "main" ]; then
  echo "Erro: este script deve ser executado na branch main"
  exit 1
fi

# Verificar se há mudanças não commitadas
if ! git diff --quiet; then
  echo "Erro: há mudanças não commitadas. Commit ou stash antes de release."
  exit 1
fi

echo "→ Atualizando Cargo.toml para v$VERSION..."
sed -i "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml

echo "→ Atualizando CHANGELOG.md..."
DATE=$(date +%Y-%m-%d)
sed -i "s/^## \[Unreleased\]/## [Unreleased]\n\n## [$VERSION] - $DATE/" CHANGELOG.md

echo "→ Commitando e tagueando..."
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump para v$VERSION"
git tag "v$VERSION"

echo "✓ Pronto! Revise o commit e push:"
echo "  git push origin main"
echo "  git push origin v$VERSION"
