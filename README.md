

## memo
- dbのpostgresもdockerで立てる
- volumeを作ったので、containerが削除されてもデータが消えることはない
    - 消えるのはdocker desktopを消したときかな？
    - なので低頻度でsql dumpみたいなのをしてバックアップファイルを取っておくのがよさそう
- 通常のmanga-update-notifierと、新規漫画データの追加や更新を担うbatchの2つのバイナリを用意する
    - 利用するbinaryが同じなので、kernel, adapterあたりを再利用できるのとdockerも共通化できてうれしいため
- rust-book-managerだとbin/app.rsでpoolの取得をしていたけど、これはregistryに任せていい気がする
    - appConfigはapp全体の設定なのでapp.rsで取る必要性はわかる


### sqlx
- cargo make migrate
  - sqlx-cli のサブコマンド migrate を使ってマイグレーションファイルを使って、データベースにスキーマ情報などを登録している
- sqlx migrate add -r start --source adapter/migrations


### リファクタ案
- episode とか title とかが全部 String なので名前を変えたい


### 運用でやりたいこと
- 途中でurlが変わったときに更新したい
- 漫画データを追加したい


### 備忘
- crawl処理が安定しなく、一度目で失敗しても二度目で成功することもある


### batch設計
- title, short_title, urlは冪等性を担保するためのファイルで一括管理したい
  - 新しくデータを加えるときや、urlが変わったときはここを使う
- deleteバッチも欲しい
  - 削除版の冪等性ファイルととらえてもよい
    - ここに記載されたデータをもしあるなら消す（なくてもエラーにならない）