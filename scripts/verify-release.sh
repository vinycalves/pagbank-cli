#!/usr/bin/env bash
set -euo pipefail

# Verifica se uma release foi publicada em todos os canais.
# Uso: ./scripts/verify-release.sh 0.2.0
# Opcional: CHECK_LOCAL=1 para validar também o binário `pb` local.

VERSION="${1:-}"
REPO="${REPO:-vinycalves/pagbank-cli}"
TAP_REPO="${TAP_REPO:-vinycalves/homebrew-tap}"
AUR_PKG="${AUR_PKG:-pagbank-cli-bin}"
UA="pagbank-cli-verify-release/1.0"

if [ -z "$VERSION" ]; then
  echo "Uso: $0 <versão>"
  echo "Exemplo: $0 0.2.0"
  exit 1
fi
VERSION="${VERSION#v}"

FAILURES=0
ok() { echo "✅ $*"; }
fail() { echo "❌ $*"; FAILURES=$((FAILURES + 1)); }

# --- GitHub Release ---
if RELEASE_JSON=$(curl -fsSL -A "$UA" "https://api.github.com/repos/$REPO/releases/tags/v$VERSION" 2>/dev/null); then
  ok "GitHub Release v$VERSION existe"
  ASSETS=$(echo "$RELEASE_JSON" | jq -r '.assets[].name')
  for asset in \
    "pb-x86_64-unknown-linux-gnu.tar.gz" \
    "pb-aarch64-unknown-linux-gnu.tar.gz" \
    "pb-x86_64-apple-darwin.tar.gz" \
    "pb-aarch64-apple-darwin.tar.gz" \
    "pb-x86_64-pc-windows-msvc.tar.gz" \
    "pagbank-cli_${VERSION}-1_amd64.deb" \
    "pb-${VERSION}-1.x86_64.rpm"; do
    if echo "$ASSETS" | grep -qx "$asset"; then
      ok "asset $asset"
    else
      fail "asset ausente: $asset"
    fi
  done
else
  fail "GitHub Release v$VERSION não encontrada"
fi

# --- crates.io ---
for crate in pagbank-sdk pagbank-cli; do
  if curl -fsSL -A "$UA" "https://crates.io/api/v1/crates/$crate/$VERSION" >/dev/null 2>&1; then
    ok "crates.io $crate $VERSION"
  else
    fail "crates.io $crate $VERSION não publicado"
  fi
done

# --- AUR (o RPC pode ficar em cache por alguns minutos) ---
AUR_VERSION=""
for _ in 1 2 3 4 5 6; do
  AUR_VERSION=$(curl -fsSL -A "$UA" "https://aur.archlinux.org/rpc/v5/info?arg[]=$AUR_PKG" 2>/dev/null \
    | jq -r '.results[0].Version // empty')
  case "$AUR_VERSION" in
    "$VERSION"-*) break ;;
  esac
  sleep 15
done
if [ -n "$AUR_VERSION" ] && [ "${AUR_VERSION%%-*}" = "$VERSION" ]; then
  ok "AUR $AUR_PKG $AUR_VERSION"
else
  fail "AUR $AUR_PKG está em '${AUR_VERSION:-desconhecido}', esperado $VERSION-*"
fi

# --- Homebrew tap ---
if FORMULA=$(curl -fsSL -A "$UA" "https://raw.githubusercontent.com/$TAP_REPO/main/Formula/pb.rb" 2>/dev/null); then
  if echo "$FORMULA" | grep -q "version \"$VERSION\""; then
    ok "Homebrew $TAP_REPO Formula/pb.rb $VERSION"
  else
    fail "Homebrew $TAP_REPO não está na versão $VERSION"
  fi
else
  fail "Homebrew $TAP_REPO/Formula/pb.rb não encontrada"
fi

# --- Binário local (opcional) ---
if [ "${CHECK_LOCAL:-0}" = "1" ]; then
  if command -v pb >/dev/null 2>&1; then
    LOCAL_VERSION=$(pb --version | awk '{print $2}')
    if [ "$LOCAL_VERSION" = "$VERSION" ]; then
      ok "pb local $LOCAL_VERSION"
    else
      fail "pb local está em $LOCAL_VERSION, esperado $VERSION"
    fi
  else
    fail "pb não encontrado no PATH"
  fi
fi

echo
if [ "$FAILURES" -eq 0 ]; then
  echo "Deploy v$VERSION verificado com sucesso."
else
  echo "Deploy v$VERSION com $FAILURES problema(s)."
  exit 1
fi
