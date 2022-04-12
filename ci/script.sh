set -euxo pipefail

main() {
    cargo check --target $TARGET --features "$FEATURES"

    if [ $TRAVIS_RUST_VERSION = nightly ]; then
        cargo check --target $TARGET --features "inline-asm,$FEATURES"
    fi
}

main
