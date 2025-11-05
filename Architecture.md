# What's this
本ツール全体の設計に関するドキュメント

# ChatGPT相談
- https://chatgpt.com/share/6906353c-a174-800e-841e-70c2903fa485

# 概要
- メイン機能
  - 任意のWeb漫画の最新話の更新があるかをチェックし、更新されていれば適切なところへ通知する
- サブ機能
  - 監視する漫画情報の追加/削除/更新

# input
- 漫画にはそれが記載されているurlがある
- 漫画のurlから欲しいのはその最新話を取得することだけ
- 漫画ごとにその取得ロジックが違うかというとそうではなく、
  同じポータルサイトだと同じロジックで取れることが多い
  - e.g. ジャンプ系、ヤンジャン系、ヤングエース系、カドコミ系、etc
  - なので、このポータルの類型ごとに集計ロジックを持っていて、各漫画はどれかの類型に紐づく形になる
  - 情報の持ち方はどうしよう
    - おそらく、mangaに、ポータルのIDを外部キーとして持たせるのがいい
    - ソースコードには、各ポータルごとの最新話取得ロジックを定義する必要がある
      - これをDBに書くことはあまり現実的ではない
- 通知は、漫画のタイトルとurlと最新話数だけあればいい
  - 通知の方法はinfra層に任せるけど、今のところ以下の2option
    - discord
    - 標準出力
- notificationは集約にせず、検知したというドメインイベントとして扱う

# 疑問点
- 上述のような事情を鑑みると、どこまでをドメインモデルとして組み込むべきだろうか？
  - 漫画がどのロジック(ポータルサイトタイプ)で情報を得るかはドメインの話？
- 各ドメインモデルはどの情報まで持つべきだろうか

# ドメインモデル
- manga
  - title
- notification
  - manga_title
  - url

# パッケージ構成
とりあえず以下の感じで考えている。追加やおすすめがあれば教えてほしい
- domain(集約ベースで切る)
  - manga
  - portal_kind
  - last_ep_detected(doamin event)
- infrastructure(Port実装の性質で切る)
  - repository
    - manga
  - fetcher(最新話の情報を取得する)
    - jump
    - yung_jump
    - kadokomi
    - webdriver(コネクションプール管理する)
  - notifier
    - discord
    - stdout
- application
  - usecase
    - create_manga
    - delete_manga
    - update_manga
  - port
    - manga_repository
    - laste_ep_fetcher
    - notifier
      - 非同期チャネルを使って、LatestEpUpdatedDetectedイベントを監視する形にする
    - clock
      - now を返すメソッドを持つだけの trait
      - for test easily
- registry(DIコンテナ)
- shared
- server
  - apiサーバーとして起動する際のエンドポイント
- batch
  - 本丸となる漫画更新検査など、一回きりの実行のエンドポイントを集める
