#!/usr/bin/env sh
set -eu

CERTS_DIR="certs"

mkdir -p "$CERTS_DIR"
chmod 700 "$CERTS_DIR"

export TRUST_STORES=none

bunx mkcert create-ca \
  --key "$CERTS_DIR/ca.key" \
  --cert "$CERTS_DIR/ca.crt"

bunx mkcert create-cert \
  --ca-key "$CERTS_DIR/ca.key" \
  --ca-cert "$CERTS_DIR/ca.crt" \
  --key "$CERTS_DIR/server.key" \
  --cert "$CERTS_DIR/server.crt" \
  --domains postgres localhost 127.0.0.1 ::1

bunx mkcert create-cert \
  --ca-key "$CERTS_DIR/ca.key" \
  --ca-cert "$CERTS_DIR/ca.crt" \
  --key "$CERTS_DIR/client.key" \
  --cert "$CERTS_DIR/client.crt" \
  --domains root

chmod 600 "$CERTS_DIR"/*.key
