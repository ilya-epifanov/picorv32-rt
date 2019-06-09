set -euxo pipefail

main() {
    cargo check --target $TARGET --features "$FEATURES"

    if [ $TRAVIS_RUST_VERSION = nightly ]; then
        cargo check --target $TARGET --features "inline-asm,$FEATURES"
    fi

    if [ $TARGET = x86_64-unknown-linux-gnu ]; then
        ./check-blobs.sh
    fi
}

main
