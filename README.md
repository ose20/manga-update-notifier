# 漫画更新検知ツール

[![CI](https://github.com/ose20/manga-update-notifier/actions/workflows/ci.yml/badge.svg)](https://github.com/ose20/manga-update-notifier/actions/workflows/ci.yml)

登録したWeb漫画の更新状況をチェックし、更新されていれば適当な通知チャネルに報告をするツール。

## 利用方法(for users)
### 必要なツール
以下のツールがインストールされていることを前提とする。

#### **Rust**
- 概要
  - 本プログラムがRustで書かれており、それをコンパイル&実行する必要があるため
- インストール方法
  - [公式で案内されている方法](https://rust-lang.org/tools/install/)などを用いてインストールする（一瞬&簡単）

#### **cargo-make**
- 概要
  - 本ツールのタスクランナーとして利用している
  - コンテナの管理、2つのバイナリモードの実行、環境変数の設定、各種タスク間の依存関係の整理などを主にやっている
  - [公式へのリンク](https://github.com/sagiegurari/cargo-make)
- インストール方法
  ```sh
  cargo install cargo-make
  ```

#### **Docker**
- 概要
  - 検知対象の漫画情報を永続化するRDB(PostgresSQL)やWebクロールに使うSeleniumなどのミドルウェアはDocker Composeで管理する
    - そのため、`docker compose`コマンドが使える環境を想定する
  - PostgreSQL、Seleniumの直接のインストールは不要
- インストール方法
  - [公式リンク](https://docs.docker.com/compose/install/)から自身の環境に合わせて利用する

### 必要なファイル
本リポジトリのプロジェクトルートに以下の内容の`secret.env`というファイルを作成する
```sh
DISCORD_BOT_TOKEN="${利用者のDiscordのBotトークン}"
DISCORD_CHANNEL_ID="${最新話の更新に使うチャンネルのID}"
DISCORD_ERR_CHANNEL_ID="${エラーの送信に使うチャンネルのID}"
```
ここで必要な情報の取得方法は次節で解説する。

### Discord Botと通知先チャンネル
本ツールでは通知にDiscordを使っているので、その通知を担うDiscord Botと、それが通知に使うチャンネルが必要になる。
- Discord Botの作成とTokenの取得
  - [公式ドキュメント](https://discord.com/developers/docs/tutorials/hosting-on-cloudflare-workers#adding-bot-permissions)の`Creating an app on Discord`のセクションを参考に、Discordのアプリケーションアカウント(bot)を作成する
  - 作成したアカウントのダッシュボードからBotトークンを取得する
    - 本ドキュメント作成時は、ダッシュボードの左側のペイン > Bot > Tokenで到達可能
- 通知に使いたいチャンネルを作成し、先ほど作成したBotを招待する
  - エラー用チャンネルを分けたい場合は同様に作成しBotを招待する
- 上記チャンネルのIDを取得する
  - 方法自体は単純なので適宜やり方を調べる

以上の手順により、前節のファイル作成に必要な3つのデータが取得できた。

### 実行方法
#### サーバーモード
追跡対象の漫画情報の作成、編集、削除ができるAPIサーバーを立ち上げるモードは次のコマンドで実行する。
```sh
cargo make run-server
```

#### バッチモード
DBに登録された漫画の更新状況を調べて、更新があったら通知をするワンショットのプログラムを実行する。
```sh
cargo make run-notifier
```


## 未整備


### memo
- dbのpostgresもdockerで立てる
- volumeを作ったので、containerが削除されてもデータが消えることはない
    - 消えるのはdocker desktopを消したときかな？
    - なので低頻度でsql dumpみたいなのをしてバックアップファイルを取っておくのがよさそう
- rust-book-managerだとbin/app.rsでpoolの取得をしていたけど、これはregistryに任せていい気がする
    - appConfigはapp全体の設定なのでapp.rsで取る必要性はわかる


### sqlx
- cargo make migrate
  - sqlx-cli のサブコマンド migrate を使ってマイグレーションファイルを使って、データベースにスキーマ情報などを登録している
- sqlx migrate add -r start --source adapter/migrations


## Todo
- registryの調整
  - 起動するバイナリによって必要な要素が違うことの反映
    - 特に server 起動するときに chromedriver を起動したくない
- ↑の問題を解決する前にselenium container消えない問題の原因究明
  - run-hoge したあとに compose-down しても selenium が消えず、docker rm -f しないといけないのはなぜ
    - 上の問題解決しちゃっても大丈夫かも
      - だめだ
        - run-notifier2連続は大丈夫だけどrun-server -> run-notifierだと registry が作れない
          - もしかして greceful shutdown してないから説ある？
        - run-notifier後でもcompose-downで消えないけど、そのあとrun-notifierしても別に困らない
          - run-notifier -> run-serverはいけるので、やっぱりgraceful shutdownしてないからかも
- webdriverが起動しきる前にやってる
2025-11-08T21:51:12.758363633+09:00 ERROR batch/src/main.rs:10: Application error: The WebDriver request returned an error: error sending request for url (http://localhost:4444/session)
  - healthcheck設定してるけどうまくいってないっぽい
- 最適化
  - rssを提供しているサイトはwebdriver使わないのでpoolから拝借しないようにできるとうれしい
- rssタイプ
  - 最新話をクロールする場所と、実際に最新話が乗っている場所が違う
- テストさぼってるのでかく
- 実際のcrawlのテストをする環境の整備