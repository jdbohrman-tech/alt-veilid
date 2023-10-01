@echo off
setlocal

REM #############################################

PUSHD %~dp0
SET ROOTDIR=%CD%
POPD

IF NOT DEFINED ProgramFiles(x86) (
    echo This script requires a 64-bit Windows Installation. Exiting.
    goto end
)

FOR %%X IN (protoc.exe) DO (SET PROTOC_FOUND=%%~$PATH:X)
IF NOT DEFINED PROTOC_FOUND (
    echo protobuf compiler ^(protoc^) is required but it's not installed. Install protoc 23.2 or higher. Ensure it is in your path. Aborting.
    echo protoc is available here: https://github.com/protocolbuffers/protobuf/releases/download/v23.2/protoc-23.2-win64.zip
    goto end
)

FOR %%X IN (capnp.exe) DO (SET CAPNP_FOUND=%%~$PATH:X)
IF NOT DEFINED CAPNP_FOUND (
    echo capnproto compiler ^(capnp^) is required but it's not installed. Install capnp 1.0.1 or higher. Ensure it is in your path. Aborting.
    echo capnp is available here: https://capnproto.org/capnproto-c++-win32-1.0.1.zip
    goto end
)

FOR %%X IN (cargo.exe) DO (SET CARGO_FOUND=%%~$PATH:X)
IF NOT DEFINED CARGO_FOUND (
    echo rust ^(cargo^) is required but it's not installed. Install rust 1.65 or higher. Ensure it is in your path. Aborting.
    echo install rust via rustup here: https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe
    goto ends
)

echo Setup successful
:end
ENDLOCAL
