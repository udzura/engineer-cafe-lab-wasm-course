----
marp: true
title: "WebAssemblyでScratchプラグインを作ろう!"
description: "At Engineer Cafe Lab Fukuoka 2024/10"
header: "WebAssemblyでScratchプラグインを作ろう!"
footer: "#1 Our First WebAssembly Run"
theme: ecl
image: https://udzura.jp/engineer-cafe-lab-wasm-course/2024-25/01_whatiswasm/ogp.png#TODO
paginate: true
----

<!--
_class: hero
-->

# WebAssemblyで<br>Scratchプラグインを作ろう!

## #1 Our First WebAssembly Run

----

# 今日やるゴール

- WebAssemblyとは？
- WASM のバイナリ構造・セクションについて（初級）
- 手元でWASMを動かす
  - その1
  - その2 / WASIの触り
- ブラウザで動かす
- importとexportの話をする
  - ブラウザで連携するには？

----

<!--
_class: hero
-->

# WebAssembly(WASM)の概要

----

# WebAssembly ってそもそも何？

- なんか... ブラウザで動くやつ...
- 「ブラウザ上でJS以外の言語を動かすことができる技術」
  - 最近使われてるらしい
  - Ruby、Python、Kotlin、他色々対応しつつあるらしい
- WASMとも呼ばれる。だいたい同じものを指す

----

# 具体的なユースケースから

- Unity3d
- Goolge Meet
- Figma（[高速化の事例](https://www.figma.com/ja-jp/blog/webassembly-cut-figmas-load-time-by-3x/)）
- Linux on browser, ...

----

# Unity3d

- 3Dゲームをブラウザで動かすことができる
  - そこでWASMを使っている
- https://docs.unity3d.com/ja/2022.1/Manual/webgl-native-plugins-with-emscripten.html

----

# Goolge Meet

- ビデオ通話の背景ぼかしに利用
- https://developers-jp.googleblog.com/2023/05/webassembly.html

![bg right w:500](./meet.png)
　

----

# Linux on Browser

- [webvm.io](https://webvm.io/)

![bg right w:600](./webvm.png)

----

# WASMが得意なこと

- 高速な処理
- 言語を選ばず実装可能
- ポータブル（ブラウザでもサーバでも組み込んで動く）

----

# ブラウザの他で動く例

- wasmで設定を書く（ロードバランサーのenvoyほか）
- wasmプログラムををコンテナとして動かす
  - [kubernetesの下でwasmを動かす](https://krustlet.dev/)

----

<!--
_class: hero
-->

# WASM のバイナリ構造とセクション

- 概要の次でいきなりバイナリの話かよ！

----

# WASM のサンプルバイナリ

- とりあえずダウンロードしてみよう

```
wget ...
```

----

# WASM の中身を確認するコマンド

----

# WASM のセクション

----

# 例えばimport？

----

# セクションの例

- 代表的なもののみ

----

<!--
_class: hero
-->

# WASM を作って動かそう

----

# My first project

----

# ビルド設定を少し編集する

----

# 関数を実装してみよう


----

# ビルドしよう

----

# wasmtime で動作確認する

----

# もう一つの動かし方

----

# この場合のビルド設定


----

# 今度は「main」を実装しよう

----

# ビルドしよう

----

# wasmtimeで動かす

----

# 2つのwasmバイナリの違い



----

# WASI？ ワシには難しくて...



----

<!--
_class: hero
-->

# ブラウザで動かそう


----

# first project の方のバイナリを使う

----

# index.html を作ろう


----

# instanciateとはなんぞや？

----

# 手元にサーバを立てて確認しよう

```
cd

# 手元にRubyが入ってる人はこっちでもOKです
# 講師はRubyの方が慣れてるので手癖でこっちを叩きますが、お好きな方で
ruby -run -e httpd -- .
```

----

# fibを計算できました！


----

<!--
_class: hero
-->

# importとexport

----

# 少し高度な話に入ります


----

# importとexportを使うコードを書いてみよう


----

# まずはビルドしてみよう

----

# セクションを確認してみよう


----

# これをブラウザで使うには？

- 前のコードだとこういうエラーが出る

----

# importObject とは？


----

# 「コールバック」をwasmに渡してみよう

----

# 動作確認

----

<!--
_class: hero
-->

# まとめ

----

# 今日のまとめ

## 以下のような内容を学んだはず

- WASM
- WASMのビルドの仕方
- WASMの動かし方（コマンド、ブラウザ）
- 関数のimport/exportの基本

----

# 次回

- #2 WebAssembly モジュールとブラウザを連携させよう
  - <span style='font-size: 30pt'>予定: 11/XX(TBA) 14:00 start</span>
  - キーワード:
    - 文字列
    - 線形メモリ
