@echo off
setlocal

set "CERTS_DIR=certs"

if not exist "%CERTS_DIR%" mkdir "%CERTS_DIR%"

set "TRUST_STORES=none"

bunx mkcert create-ca --key "%CERTS_DIR%\ca.key" --cert "%CERTS_DIR%\ca.crt"
bunx mkcert create-cert --ca-key "%CERTS_DIR%\ca.key" --ca-cert "%CERTS_DIR%\ca.crt" --key "%CERTS_DIR%\server.key" --cert "%CERTS_DIR%\server.crt" --domains postgres localhost 127.0.0.1 ::1
bunx mkcert create-cert --ca-key "%CERTS_DIR%\ca.key" --ca-cert "%CERTS_DIR%\ca.crt" --key "%CERTS_DIR%\client.key" --cert "%CERTS_DIR%\client.crt" --domains root

echo Certificates generated in %CERTS_DIR%
endlocal
