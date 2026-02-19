#/bin/bash

echo "where do you want to install it?"
echo "\$HOME/.local/bin/lap_tap"
echo -n "> "
path="$HOME/.local/bin/lap_tap"
read input_path
if [ -z "$path" ]; then
  path=${input_path}
fi

mkdir -p "${path}"
echo "created install folder at ${path}"

echo "start installing to ${path}"

echo "Downloading resources from github ..."

curl -L https://github.com/Uliboooo/lap_tap/releases/latest/download/lap_tap -o "${path}/lap_tap"
curl -L https://github.com/Uliboooo/lap_tap/releases/latest/download/lap_tap_helper -p "${path}/lap_tap_helper"
curl -L https://github.com/Uliboooo/lap_tap/releases/latest/download/resources -o "${path}/resouces"

echo "please add ${path} to \$PATH"
