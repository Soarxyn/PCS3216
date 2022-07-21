if command -v python3
then
    py="python3"
else
    py="python"
fi

case "$(uname -s)" in
    Linux*)     source .env/bin/activate;;
    Darwin*)    machine=Mac;;
    *)  cd .env/Scripts
        . activate
        cd ../..;;
esac

eval " $py src/main.py"
deactivate
