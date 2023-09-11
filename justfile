help: 
    echo "help"

cli:
  -export RUSTFLAGS="-C link-arg=$(clang -print-libgcc-file-name)"
  cargo build --bin cli --release 
  rm output/cli
  cp target/release/cli output/ 
  @echo "\n\nrun 'sudo ./cli b30'"

web:
  -export RUSTFLAGS="-C link-arg=$(clang -print-libgcc-file-name)" 
  cargo build --bin web --release 
  rm output/web
  cp target/release/web output/ 
  @echo "\n\nrun 'sudo ./web' and visit 'localhost:8000/b30'"
