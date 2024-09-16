----
marp: true
title: "WebAssemblyでScratchプラグインを作ろう!"
description: "At Engineer Cafe Lab Fukuoka 2024/10"
header: "WebAssemblyでScratchプラグインを作ろう!"
footer: "#0 Install Day"
theme: ecl
image: https://udzura.jp/engineer-cafe-lab-wasm-course/2024-25/00_installday/ogp.png#TODO
paginate: true
----

<!--
_class: hero
-->

# WeWebAssemblyで<br>Scratchプラグインを作ろう!

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

# 本講座の特徴#　2

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

# そういうわけで...

- 難易度が高い、一緒に講座を作りたい、ということで
  - <strong style='font-size: 40pt;'>オフラインでの参加を強く推奨します</strong>
- ただし、資料は全て公開（録画もあるそうです）、Discordで適切な内容であれば質問は可能です。自学にも対応した形にはなっています

- <small>また、テイカー気質の方は少し想定している参加者層とは違うこともご了承ください</small>

<!-- 講座というより、学生の自主ゼミみたいな感じかもしれない -->

----

<!--
_class: hero
-->

# 環境構築

----

<!--
_class: normal
-->

# はじめに

- 今日の資料は以下にもアップされています
  - 元テキスト: https://github.com/udzura/engineer-cafe-lab-wasm-course
    - `00_installday` ディレクトリ
  - スライド: https://udzura.jp/engineer-cafe-lab-wasm-course/2024-25/00/
- `[udzura github wasm course]` で検索？

----

<!--
_class: normal
-->

# インストール/セットアップする環境

- 前提となるツール
  - Mac: Homebrew
  - Windows: WSL2
- ブラウザ
- Rustの開発環境
- WASMの開発環境
- エディタ

----

<!--
_class: normal
-->

# Mac: Homebrewのセットアップ

- URL

----

<!--
_class: normal
-->

# Windows: WSL2環境の立ち上げ


----

<!--
_class: normal
-->

# ブラウザ

- （もしインストールしていなければ） Chrome をインストールしてください
- URL

----

<!--
_class: normal
-->

# Rustの開発環境

- Rustの環境一式をセットアップしておきましょう

```
```

- Windowsの方: この作業はWSL2のコンソールで実行してください

----

<!--
_class: normal
-->

# Rustの開発環境（続き）

- RustでWASMをビルドできるよう以下のコマンドも追加で打ちます

```
```

----

<!--
_class: normal
-->

# Rustの動作確認

----

<!--
_class: normal
-->

# WASMの開発環境/Mac

- Macの方...
  - Homebrew経由で簡単にインストールできます
  - **WasmEdge**
  - **WABT** (WebAssembly Binary Toolkit)
  - **wasm-tools**

----

<!--
_class: normal
-->

# WASMの開発環境/Win or Linux

- Windowsの方...
  - WSL2 の方に以下をインストールします。Linux用のバイナリを選択することになるでしょう
  - **WasmEdge**
  - **WABT** (WebAssembly Binary Toolkit)
  - **wasm-tools**

----

<!--
_class: normal
-->

# WASMツールの動作確認

- 以下のwasmバイナリを用意してみましたので、ダウンロードしてください

----

<!--
_class: normal
-->

# エディタ

- 基本的に自由です
- こだわりがなければ VSCode が良いかと思います
- VSCodeの場合、以下の拡張を入れます

----

<!--
_class: normal
-->

# 今日のまとめ

- 必要と思われるツールをまずはセットアップしました
  - 今後の展開で随時追加するかもしれません
- 一緒にWASM講座、作っていきましょう

----

<!--
_class: normal
-->

# 次回

- #1 小さな WebAssembly モジュールを動かそう