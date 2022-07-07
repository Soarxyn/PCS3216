if [ "$1" == "-d" ]; then
    cargo build
    case "$(uname -s)" in
        Linux*)     ln -sf ./target/debug/libsisprog.so ./sisprog.so;;
        Darwin*)    ln -sf ./target/debug/libsisprog.dylib ./sisprog.so;;
        *)          ln -sf ./target/debug/sisprog.dll ./sisprog.pyd;;
    esac
else
    cargo build --release
    case "$(uname -s)" in
        Linux*)     ln -sf ./target/release/libsisprog.so ./sisprog.so;;
        Darwin*)    ln -sf ./target/release/libsisprog.dylib ./sisprog.so;;
        *)          ln -sf ./target/release/sisprog.dll ./sisprog.pyd;;
    esac
fi
