#!/bin/bash

# スクリプトのディレクトリに移動
# $0 現在実行しているスクリプトの名前
cd "$(dirname "$0")"

# 適宜 logrotate の設定をしてください
LOG_FILE="/var/log/manga_update_notifier/notifier.log"

CARGO_PATH="$HOME/.cargo/bin/cargo"

"$CARGO_PATH" make run-notifier && "$CARGO_PATH" make stop-chromedriver >> "$LOG_FILE" 2>&1