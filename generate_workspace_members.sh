#!/bin/bash
# このスクリプトは、プロジェクトルート以下でCargo.tomlが存在するディレクトリ（ルート除く）
# を検出し、Cargo ワークスペースの members セクション用のリストを出力します。

# -mindepth 2 でルートのCargo.tomlを除外し、
# sed で先頭の"./"を削除して、見やすい相対パスに変換します。
members=$(find . -mindepth 2 -type f -name "Cargo.toml" \
    | xargs -n1 dirname \
    | sed 's|^\./||' \
    | sort -u)

echo "[workspace]"
echo "members = ["
for m in $members; do
    echo "    \"$m\","
done
echo "]" 