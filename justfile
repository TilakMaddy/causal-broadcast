run-nodes:
    @echo "Running nodes.."
    npx concurrently "RUST_LOG=trace cargo su 1" "RUST_LOG=trace cargo su 2" "RUST_LOG=trace cargo su 3"
