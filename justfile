help: 
    echo "help"

cli:
  
  export RUSTFLAGS+=" -C link-arg=$(clang -print-libgcc-file-name)"  
  cargo build --bin cli --release 
  cp target/release/cli output/ 
  cd output 
  sudo ./cli
