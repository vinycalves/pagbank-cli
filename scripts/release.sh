#!/usr/bin/env bash
set -euo pipefail

if [ $# -ne 1 ]; then
  echo "Uso: $0 <versão>"
  echo "Exemplo: $0 0.2.0"
  exit 1
fi

VERSION="$1"
BRANCH=$(git branch --show-current)

# Verificar se há mudanças não commitadas
if ! git diff --quiet; then
  echo "Erro: há mudanças não commitadas. Commit ou stash antes de release."
  exit 1
fi

if [ "$BRANCH" = "main" ]; then
  RELEASE_BRANCH="chore/release-v$VERSION"
  echo "→ main é protegida; criando branch $RELEASE_BRANCH..."
  git checkout -b "$RELEASE_BRANCH"
  BRANCH="$RELEASE_BRANCH"
fi

case "$BRANCH" in
  chore/release-v*) ;;
  *)
    echo "Erro: execute na main ou em uma branch chore/release-v*."
    exit 1
    ;;
esac

echo "→ Atualizando Cargo.toml para v$VERSION..."
sed -i "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml

echo "→ Atualizando CHANGELOG.md..."
DATE=$(date +%Y-%m-%d)
sed -i "s/^## \[Unreleased\]/## [Unreleased]\n\n## [$VERSION] - $DATE/" CHANGELOG.md

echo "→ Commitando e enviando a branch..."
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump para v$VERSION"
git push -u origin "$BRANCH"

echo "✓ Bump enviado em $BRANCH."
echo "  1. Abra e mergeie o PR de $BRANCH para main."
echo "  2. Depois do merge, crie a tag com:"
echo "     ./scripts/tag-release.sh $VERSION"
