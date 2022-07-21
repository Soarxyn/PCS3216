if [ "$1" == "-d" ]
then
    develop="maturin develop"
else
    develop="maturin develop -r"
fi

if command -v python3
then
    py="python3"
else
    py="python"
fi

eval " $py -m venv .env"

case "$(uname -s)" in
    Linux*)
        source .env/bin/activate;;
    Darwin*)
        machine=Mac;;
    *) cd .env/Scripts
        . activate
        cd ../..;;
esac

eval " $py -m pip install -r requirements.txt"
eval " $develop"
deactivate
