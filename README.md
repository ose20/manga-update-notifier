
[![CI](https://github.com/ose20/manga-update-notifier/actions/workflows/ci.yml/badge.svg)](https://github.com/ose20/manga-update-notifier/actions/workflows/ci.yml)

## 動かし方


## memo
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