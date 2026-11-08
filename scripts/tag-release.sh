#!/usr/bin/env bash
set -euo pipefail

if [ $# -ne 1 ]; then
  echo "Uso: $0 <versão>"
  echo "Exemplo: $0 0.2.0"
  exit 1
fi

VERSION="$1"
TAG="v$VERSION"

if [ "$(git branch --show-current)" != "main" ]; then
  echo "Erro: este script deve ser executado na branch main."
  exit 1
fi

git pull --ff-only

CARGO_VERSION="$(grep '^version = ' Cargo.toml | head -1 | cut -d'"' -f2)"
if [ "$CARGO_VERSION" != "$VERSION" ]; then
  echo "Erro: Cargo.toml está em $CARGO_VERSION, esperado $VERSION."
  exit 1
fi

if git rev-parse "$TAG" >/dev/null 2>&1; then
  echo "Erro: a tag $TAG já existe."
  exit 1
fi

git tag -a "$TAG" -m "Release $TAG"
git push origin "$TAG"

echo "✓ Tag $TAG enviada. O workflow de release foi disparado."
echo "  Acompanhe com: gh run watch"
