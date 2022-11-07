set -euxo pipefail

main() {
    cargo check --target $TARGET --features "$FEATURES"
}

main
