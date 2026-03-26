@echo off
setlocal

set "CERTS_DIR=certs"

if not exist "%CERTS_DIR%" mkdir "%CERTS_DIR%"

set "CAROOT=%CERTS_DIR%"
set "TRUST_STORES=none"

bunx mkcert -cert-file "%CERTS_DIR%\server.crt" -key-file "%CERTS_DIR%\server.key" postgres localhost 127.0.0.1 ::1
bunx mkcert -client -cert-file "%CERTS_DIR%\client.crt" -key-file "%CERTS_DIR%\client.key" root

echo Certificates generated in %CERTS_DIR%
endlocal
