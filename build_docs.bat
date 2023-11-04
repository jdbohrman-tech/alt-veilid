@echo off
setlocal

SET BUILD_DOCS=1
cargo rustdoc -p veilid-core
cargo rustdoc -p veilid-tools

endlocal