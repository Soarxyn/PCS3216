if [ "$1" == "-d" ]; then
    cargo build
    case "$(uname -s)" in
        Linux*)     ln -sf ./target/debug/libsisprog.so ./sisprog.so;;
        Darwin*)    ln -sf ./target/debug/libsisprog.dylib ./sisprog.dylib;;
        *)          ln -sf ./target/debug/libsisprog.dll ./sisprog.dll;;
    esac
else
    cargo build --release
    case "$(uname -s)" in
        Linux*)     ln -sf ./target/release/libsisprog.so ./sisprog.so;;
        Darwin*)    ln -sf ./target/release/libsisprog.dylib ./sisprog.dylib;;
        *)          ln -sf ./target/release/libsisprog.dll ./sisprog.dll;;
    esac
fi
