# OSC4AU2

[OpenSoundControl (OSC)](https://opensoundcontrol.stanford.edu/) を使って AviUtl2 のパラメーターを変化させる時間制御スクリプト

# 仕様

`UDP:9000` で OSC メッセージを受信します

アドレスは `/osc4au2/<ID>` です（値はFloatのみ）

`<ID>` は、時間制御の `ID` パラメータと連動しています

時間制御の範囲は「開始点の値」と「最初の中間点の値」の間となります

例） `/osc4au2/0/` へ `0.5` を送信すると、 `ID` が `0` に設定されている時間制御は「開始点の値」と「最初の中間点の値」のちょうど中間の値をとります

# インストール

以下の方法が提供されています：

- AviUtl2カタログからインストール
- リリース から `.au2pkg.zip` 形式のファイルを入手
- 手動でビルド（後述）

## [AviUtl2カタログ](https://github.com/Neosku/aviutl2-catalog) からインストール（おすすめ）

上記アプリをインストールし、検索欄に `OSC4AU2` と入力してください

出てきた `OSC4AU2` の `インストール` ボタンを押すと自動で導入されます

## `.au2pkg.zip` ファイルからインストール

[リリース](../../releases/latest) から `.au2pkg.zip` 形式のファイルをダウンロードし、 AviUtl2 のプレビュー画面にドラッグ＆ドロップしてください

インストール完了後、自動で再起動されます

## 手動でビルド

MSVC と Rust が必要です

### 手順

1. このリポジトリをクローン
2. `cargo build`
3. ファイルをコピー＆リネーム
   - `target/debug (または release)/osc4au2.dll` → `Script/OSC4AU2.mod2`
   - `script/OSC4AU2.tra2` → `Script/OSC4AU2.tra2`
