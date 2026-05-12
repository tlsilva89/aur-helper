#!/usr/bin/env bash
set -e

TARGET="x86_64-unknown-linux-gnu"
OUTPUT="./linux-publish"
BINARY="aur-helper"

printf '\033[1;36m[1/3]\033[0m Compilando em modo Release...\n'
cargo build --release --target "$TARGET"

printf '\033[1;36m[2/3]\033[0m Preparando diretório de publicação...\n'
rm -rf "$OUTPUT"
mkdir -p "$OUTPUT"

printf '\033[1;36m[3/3]\033[0m Copiando binário...\n'
cp "target/$TARGET/release/$BINARY" "$OUTPUT/$BINARY"

SIZE=$(du -sh "$OUTPUT/$BINARY" | cut -f1)

printf '\033[1;32m\n✔  Publicado com sucesso!\033[0m\n'
printf '   Arquivo  : %s/%s\n' "$OUTPUT" "$BINARY"
printf '   Tamanho  : %s\n' "$SIZE"
printf '\n\033[1;33mPara publicar uma release no GitHub:\033[0m\n'
printf '   git tag v1.0.0\n'
printf '   git push origin v1.0.0\n\n'
