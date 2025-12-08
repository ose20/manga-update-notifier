// PostCSS に使うプラグインを教える設定
// PostCSS は CSS を構文解析して、プラグインで変換するための土台
// Tailwind は PostCSS プラグインとして動く CSS 生成ツール
module.exports = {
    plugins: {
        // v4ではこう書く
        '@tailwindcss/postcss': {},
        // ベンダープレフィックスを自動付与するプラグイン
        // 古いブラウザ向けの prefix を自動で追加してくれる
        autoprefixer: {},
    },
}