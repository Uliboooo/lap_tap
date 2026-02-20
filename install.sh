#!/bin/bash

set -e

readonly DEFAULT_PATH="$HOME/.local/bin/lap_tap"
readonly GITHUB_RELEASE_URL="https://github.com/Uliboooo/lap_tap/releases/latest/download/lap_tap.zip"

readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly NC='\033[0m'

error_exit() {
    echo -e "${RED}Error: $1${NC}" >&2
    exit 1
}

# 作業用の一時ディレクトリを作成（OSのtempを利用するのが安全）
WORK_DIR=$(mktemp -d) || error_exit "Failed to create temporary directory"
trap 'rm -rf "$WORK_DIR"' EXIT

echo "Where do you want to install it?"
echo -e "${YELLOW}Default: ${DEFAULT_PATH}${NC}"
read -p "> " input_path

# 入力が空ならデフォルトを使用、入力があればそれを採用
INSTALL_PATH="${input_path:-$DEFAULT_PATH}"

# インストール先の妥当性チェック
if [[ -z "$INSTALL_PATH" ]]; then
    error_exit "Install path cannot be empty"
fi

# インストール先の親ディレクトリが存在するか確認
parent_dir=$(dirname "$INSTALL_PATH")
if [[ ! -d "$parent_dir" ]]; then
    error_exit "Parent directory does not exist: $parent_dir"
fi

echo -e "${GREEN}Installing to ${INSTALL_PATH}...${NC}"

# フォルダ作成
mkdir -p "$INSTALL_PATH" || error_exit "Failed to create install directory"

echo "Downloading resources from GitHub..."

# zipファイルをダウンロード
if ! wget -q -O "${WORK_DIR}/lap_tap.zip" "$GITHUB_RELEASE_URL"; then
    error_exit "Failed to download from GitHub. Check your internet connection or the URL."
fi

# ダウンロードしたファイルが存在するか確認
if [[ ! -f "${WORK_DIR}/lap_tap.zip" ]]; then
    error_exit "Downloaded file not found"
fi

# 解凍
if ! unzip -q "${WORK_DIR}/lap_tap.zip" -d "${WORK_DIR}"; then
    error_exit "Failed to extract zip file"
fi

# コピー前に、解凍後のディレクトリ構造を確認
if [[ ! -d "${WORK_DIR}/release" ]]; then
    error_exit "Expected directory 'release' not found in extracted files"
fi

# コピー
if ! cp -rv "${WORK_DIR}/release"/* "$INSTALL_PATH/"; then
    error_exit "Failed to copy files"
fi

echo "----------------------------------------"
echo -e "${GREEN}Installation complete!${NC}"
echo ""
echo "Please add the following to your \$PATH (e.g., in .bashrc or .zshrc):"
echo -e "${YELLOW}export PATH=\"${INSTALL_PATH}:\$PATH\"${NC}"
echo ""
echo "Then run: source ~/.bashrc  # or source ~/.zshrc"
