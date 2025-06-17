@echo off
echo Building the application in release mode...
cargo build --release
echo Build finished. The executable is located in the target/release/ directory.
pause 