#!/usr/bin/env sh
set -eu

CERTS_DIR="certs"

mkdir -p "$CERTS_DIR"
chmod 700 "$CERTS_DIR"

export CAROOT="$CERTS_DIR"
export TRUST_STORES=none

bunx mkcert \
  -cert-file "$CERTS_DIR/server.crt" \
  -key-file "$CERTS_DIR/server.key" \
  postgres localhost 127.0.0.1 ::1

if [ -f "$CERTS_DIR/rootCA.pem" ]; then
  cp "$CERTS_DIR/rootCA.pem" "$CERTS_DIR/ca.crt"
fi

bunx mkcert -client \
  -cert-file "$CERTS_DIR/client.crt" \
  -key-file "$CERTS_DIR/client.key" \
  root

chmod 600 "$CERTS_DIR"/*.key
