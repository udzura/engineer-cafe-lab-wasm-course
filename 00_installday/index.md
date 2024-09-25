----
marp: true
title: "WebAssemblyでScratchプラグインを作ろう!"
description: "At Engineer Cafe Lab Fukuoka 2024/10"
header: "WebAssemblyでScratchプラグインを作ろう!"
footer: "#0 Install Day"
theme: ecl
image: https://udzura.jp/engineer-cafe-lab-wasm-course/2024-25/00_installday/ogp.png
paginate: true
----

<!--
_class: hero
-->

# WebAssemblyで<br>Scratchプラグインを作ろう!

## #0 Install Day

----

<!--
_class: hero
-->

# 今日やるゴール

----

<!--
_class: normal
-->

# 今日やるゴール

* 本講座の概要説明
* 必要な環境の構築

----

<!--
_class: hero
-->

# 概要

----

<!--
_class: normal
-->

# 本講座の概要説明

* 何をするの？
* やることは？（言語、技術、他）
* 想定する受講者のレベル
* その他

※ 今後の説明において、時期、内容等あくまでも予定です

----

<!--
_class: normal
-->

# 本講座では何をするのか？

* WebAssembly の基本的なところを学びます（〜12月）
* WebAssembly の応用の一環で、ScratchのブロックとしてWebAssemblyで作った機能を組み込みます（2025年1月〜）

※ 本講座を通して、WebAssemblyのことを短くWASMと呼ぶことがあります

----

<!--
_class: normal
-->

# イメージ

- Scratchは自分でMODが作れるので、そこで動く何かを想定しています（予定）

![bg right w:550](./shot1.png)

----

<!--
_class: normal
-->

# 具体的なやること

- 今回を含め **全6回** の予定です（月一回程度のオフライン開催）
  * #0 インストールデイ（今回）
  * #1 小さな WebAssembly モジュールを動かそう
  * #2 WebAssembly モジュールとブラウザを連携させよう
  * #3 WebAssembly モジュールでより複雑なことをしよう
  * #4 Scratch をハックする方法を学ぼう
  * #5 Scratch とWebAssemblyを連携させよう


----

<!--
_class: normal
-->

# 具体的に扱う技術・環境

* ブラウザ ... 基本的に最新のChromeのみ動作確認します
* 言語 ... **Rust** を使います
  - Rust は一般的にはやや学習が大変な言語ですが、Rustの中でも難しめの概念（特にライフタイム周り）はなんとか誤魔化して進めます
  - また、JavaScript/TypeScriptも必要に応じ扱うことになります
    - 全てサンプルコードは用意します
* OS ... MacOS / Windows どちらのPCでもご参加いただけます
  - MacOS での開発を推奨します
  - Windows を選ぶ場合、ある程度の自力解決能力を期待します

----

<!--
_class: normal
-->

# 想定する受講者のレベル

- 以下のレベルを期待します
  * 最低でも一つ何かしらの普通のプログラミング言語を取り扱える
    - Scratch しか触れたことがない場合は少し難しいです
  * コマンドラインツールに触れたことがある
- Scratch 自体を使ったことがあるかどうかは問いません

----

<!--
_class: hero
-->

# 本講座の特徴

----

<!--
_class: normal
-->

# 本講座の特徴#1

<strong style='font-size: 100pt'>地味です</strong>

- 少なくとも途中まで割と地味な感じになります🥺 あらかじめご了承ください

----

<!--
_class: normal
-->

# 本講座の特徴#2

<strong style='font-size: 60pt'>難易度やや高めです</strong>

- あまり前例がない/資料がないことをやっていきます
- 進度も正直速いです
- Discord等もうまく活用したい気持ちがあります

----

<!--
_class: normal
-->

# 本講座の特徴#3

<strong style='font-size: 60pt'>一緒に調べて、作っていきます</strong>

- 講師はフロントエンドエンジニアでは**ありません**（‼️）
  - RubyをWebAssemblyで動かす等いくつか実験をしたぐらい
- 例えば質問内容にその場では答えられないことがあります
（調べることはもちろんします）
- 特に後半はReact/TypeScriptを取り扱い、むしろ識者と意見交換したい...

----

<!--
_class: normal
-->

# 本講座の免責事項...

- Windows マシンでの動作確認は十分に行っていません
  - 利用されることは制限しません
  - 場合によってかなりの部分を自分でトラブルシュートする必要があるかもしれません
    - （Macでもそうなのですが、Windowsではそうなる確率が上がります）
  - コマンドライン操作をWSL上での実行に寄せると少し楽かも？

----

<!--
_class: normal
-->

# そういうわけで...

- 難易度が高い、一緒に講座を作りたい、ということで
  - <strong style='font-size: 40pt;'>オフラインでの参加を強く推奨します</strong>
- ただし、資料は全て公開（録画もあるそうです）、Discordで適切な内容であれば質問は可能です。自学にも対応した形にはなっています

- <small>また、テイカー気質の方は少し想定している参加者層とは違うこともご了承ください</small>

<!-- 講座というより、学生の自主ゼミみたいな感じかもしれない -->

----

<!--
_class: normal
-->

# オフライン推奨の話

- もう少し解像度を上げます

----

<!--
_class: normal
-->

# トラブった時

- 質問をすると思いますが
- 実は、講師・コーチがそのものずばりの答えを知っていることは少ないです

![bg right:58% w:650](./aruaru.png)

----

<!--
_class: normal
-->

# 何ができるか？

* 学習者より**トラブルシュート**が上手なはずです
* その大きな違いは何かというと、**手数**です

![bg right:58% w:650](./wa-waa.png)

----

<!--
_class: normal
-->

## オフラインならば

* コーチと一緒にたくさんある確認事項・試行錯誤をその場で一つずつやっていくことができます

## オンラインでは...

* 数多く（場合によって10以上）の確認事項を**一気に全部**共有しないといけません
  - 情報量が多く「ワ、ワァ...」となってしまうと思います
  - かと言って一つずつやるとやり取りがとても遅いため大変です
  - しかしトラブルの原因はわからないため、見つかるまで**やれることは全部**
  確認する必要があります


----

<!--
_class: normal
-->

# そういうわけで

- オフライン推奨 ＝ 必ず起こるトラブルシュートを効率よくするため、です
- 毎回オフライン参加でなくても大丈夫ですが...
  - 困ったことがあった場合、どこかでオフラインで質問に来た方が効率が良いことも多いでしょう
  - （その場合、例えば前回範囲の質問、なども歓迎します）
  - ある程度時間がかかってもOK、ならオンライン質問も活用できます

----

<!--
_class: normal
-->

# オンラインを全面否定するわけではないです

- いろいろな事情もあります
- 選択肢は多い方がいいです
- しかし、さまざまな制約からオンライン学習では難しい要素もあるので、
ある意味で**より自律的、積極的な学習**が求められることは理解してください。
  - 自分一人でも基本的な手数を稼げるようになるとベター

----

<!--
_class: normal
-->

# FYI: オンラインでの質問のコツ

- 以下の**3項目**を意識して言語化しましょう
  - **(1) やろうとしていること・意図**
  - **(2) 実際にやったこと（コマンドなど）**
  - **(3) 結果として起こったこと・何が意図と反するか**
- **すべてのログを共有しましょう**
  - めちゃくちゃ長くても大丈夫です、 @udzura はちゃんと読み（め）ます
  - 変に忖度して削った結果重要な情報が消えているパターンの方が困ります
  - 同様に、打ったコマンド、書いたコード、検索したワードなども省略せずにありのままに伝えましょう

----

<!--
_class: normal
-->

# 参考になるページ

- よく言及されますが:
  - <span style='font-size: 28pt'>[「技術的なお問い合わせに関するガイドライン」](https://aws.amazon.com/jp/premiumsupport/tech-support-guidelines/)</span>
  - @ AWSサポートページ

----

<!--
_class: normal
-->

# いろいろ言いましたが...

- 基本楽しくやっていきましょう
- トラブルも楽しむ！（？）

----

<!--
_class: hero
-->

# 環境構築

----

<!--
_class: normal
-->

# はじめにコピペ元を掲示

- 今日の資料は以下にもアップされています
  - 元テキスト: https://github.com/udzura/engineer-cafe-lab-wasm-course
    - `00_installday` ディレクトリ
  - スライド: https://udzura.jp/engineer-cafe-lab-wasm-course/2024-25/00/
- `[udzura github wasm course]` で検索？
  - コマンドは全然コピペしちゃっていいです！

----

<!--
_class: normal
-->

# インストール/セットアップする環境

- 前提となるツール
  - ✅ Mac: Homebrew
  - ✅ Windows: WSL2
- ✅ ブラウザ: Chrome
- ✅ Rustの開発環境
- ✅ WASMの開発環境
- ✅ エディタ

----

<!--
_class: normal
-->

# Mac: Homebrewのセットアップ

- https://brew.sh/ja/ の指示の通りコマンドを実行してください

## 参考情報

- Rails Girls GuideのHomebrewセットアップの項目も参考になります
  - https://railsgirls.jp/install/macos

----

<!--
_class: normal
-->

# Windows: WSL2環境の立ち上げ

- Rails Girls Guideの「WSLの導入」を参考にして、Ubuntuアプリケーションを起動してください
  - https://railsgirls.jp/install/windows-wsl

## 更なる便利な情報
- Cドライブにあるファイルを参照したり、WSLのホームに作ったファイルを    Windows側から参照する方法は例えば以下を参考にしてください
  - https://qiita.com/quzq/items/1096c638c0d86795be13

----

<!--
_class: normal
-->

# ブラウザ

- （もしインストールしていなければ） Chrome をインストールしてください
  - https://support.google.com/chrome/answer/95346?hl=ja

----

<!--
_class: normal
-->

# Rustの開発環境

- Rustの環境一式をセットアップしておきましょう
  - https://www.rust-lang.org/ja/tools/install
- 選択肢は全てデフォルトで大丈夫です
- **PATH環境変数を設定する** のを忘れずに

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Windowsの方: この作業はWSL2のコンソールで実行してください

----

<!--
_class: normal
-->

# Rustの開発環境（続き）

- RustでWASMをビルドできるよう以下のコマンドも追加で打ちます

```
rustup target add wasm32-wasi
rustup target add wasm32-unknown-unknown
```

----

<!--
_class: normal
-->

# Rustの動作確認

```bash
cargo new test-crate --bin
cd test-crate 
cargo build
./target/debug/test-crate 

#=> Hello, world! と表示されればOK
```

----

<!--
_class: normal
-->

# WASMの開発環境/Mac

- Macの方...
  - Homebrew経由で簡単にインストールできます
  - **wasmtime**
  - **WABT** (WebAssembly Binary Toolkit)
  - **wasm-tools**

```
brew install wasmtime wabt wasm-tools
```

----

<!--
_class: normal
-->

# WASMの開発環境/Win or Linux

- Windowsの方...
  - WSL2 の方に以下をインストールします。
  - **wasmtime**
  - **WABT** (WebAssembly Binary Toolkit)
    - Linux用のバイナリを選択
  - **wasm-tools**

----

<!--
_class: normal
-->

# WASMの開発環境/Win or Linux(コマンド)

```bash
# wasmtime
curl https://wasmtime.dev/install.sh -sSf | bash
. /home/ubuntu/.bashrc
# wabt (x64前提)
wget https://github.com/WebAssembly/wabt/releases/download/1.0.36/wabt-1.0.36-ubuntu-20.04.tar.gz
tar xzf wabt-1.0.36-ubuntu-20.04.tar.gz
sudo mv wabt-1.0.36/bin/* /usr/local/bin
# wasm-tools
cargo install --locked wasm-tools
```

----

<!--
_class: normal
-->

# WASMツールの動作確認

- 以下のwasmバイナリをRustで作ってみる

```bash
# 先ほどhello worldしたプロジェクトで実行
cargo build --target wasm32-wasi
file target/wasm32-wasi/debug/test-crate.wasm

# target/wasm32-wasi/debug/test-crate.wasm: WebAssembly \
# (wasm) binary module version 0x1 (MVP)
# ... のように表示される(1行で)
```

----

<!--
_class: normal
-->

# WASMツールの動作確認(2)

- wasmtimeで実行しましょう

```bash
wasmtime target/wasm32-wasi/debug/test-crate.wasm
# Hello, world! と同じように出力される
```

- 他のツールも試しましょう

```bash
wasm-objdump -x target/wasm32-wasi/debug/test-crate.wasm || echo NG!
# NG! と出なければOK
wasm-tools validate -v target/wasm32-wasi/debug/test-crate.wasm || echo NG!
# NG! と出なければOK
```

----

<!--
_class: normal
-->

# エディタ

- 基本的に自由です
- こだわりがなければ VSCode が良いかと思います
- VSCodeの場合、以下の拡張を入れます（すごく種類があるがこれがいいみたい）
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
  - TypeScriptについてはVSCodeは組み込みでサポートされています

----

<!--
_class: normal
-->

# 今日のまとめ

- 講座の雰囲気や狙い、学習者の心得的な話をしました
- 必要と思われるツールをまずはセットアップしました
  - 今後の展開で随時追加するかもしれませんが
- 一緒にWASM講座、作っていきましょう

----

<!--
_class: normal
-->

# 次回

- #1 小さな WebAssembly モジュールを動かそう
  - <span style='font-size: 30pt'>予定: 10/27(日) 14:00 start</span>