----
marp: true
title: "WebAssemblyでScratchプラグインを作ろう!"
description: "At Engineer Cafe Lab Fukuoka 2024/12"
header: "WebAssemblyでScratchプラグインを作ろう!"
footer: "#3 WebAssembly for Our Application"
theme: ecl
image: https://udzura.jp/engineer-cafe-lab-wasm-course/2024-25/03_wasmadvanced/ogp.png
paginate: true
----

<!--
_class: hero
-->

# WebAssemblyで<br>Scratchプラグインを作ろう!

## #3 WebAssembly for Our Application

----

# もっと複雑になります

- わかる範囲でも手を動かそう
- 一歩一歩やっていく
- 年末年始にも勉強できますね...

----

# 画像処理をしよう

----

# 今日の学習の流れ

- Rustで画像処理の基本的なコードを書く
  - グレースケール化
- それをWASMにする
- また、ブラウザでカメラめいたものを実装する
- そして... それらのデータを繋ぎこむ！

----

# やることが... 多い！

- 頑張りましょう。年末だし
- 今日間に合わなくても、Discordや別日での質問を活用しよう

----

# Rustで画像処理

----

# 画像のグレースケール化

----

# 画像の「明るさ」の基本概念

- 色によって感じ方が違う
- 人間の明るさの感じ方は線形でない

----

# ガンマ補正とは？

- 人間の明るさの感じ方は線形でない
- なので、強さによって自然に見えるよう、ある程度ディスプレイでの出し方の補正をかけている
- これがガンマ補正

----

# なのでグレースケールにするときも...

- 「色によって感じ方が違う」3色をまとめるのでガンマ補正を考慮しないといけない
  - モニタでは3色それぞれに曲線がある

----

# 実際やってみよう

- 「平均値」で画像をグレースケールにする

※ サンプルコードの中で、画像のピクセル化の実装例も提示しています。

----

# Rustでの実装

- imageクレートを使う
  - なるべく低レイヤと言ったが...
  - 流石にPNGのフォーマットをパースして、までは時間がないため...
- このクレートはそのままWASMでも動く！

----

# 実装準備

----

# 実装コード

```rust
```

----

# 動作確認

----

# ブラウザで「カメラ」を実装する

----

# 「カメラ」を実装する作戦

- video要素を使うことができる
- video要素を映像のみにする
- video要素の映像フレームは、取り出してcanvas要素の中身にできる
  - `drawImage()`
- そうなれば `toDataURL()` で取り出してimgを更新可能

----

# 実装コード(html)

```html
<html>
    <head>
        <title>Camera</title>
        <script async type="text/javascript">...</script>
    </head>
    <body>
        <h1>Camera to WASM</h1>
        <div>
            <video autoplay muted playsinline id="myvideo" width="480" height="320"></video>
        </div>
        <div>
            <button type="button" id="snap">Snap!</button>
        </div>
        <div>
            <img id="destination" alt="target" style="display: none;">
        </div>
    </body>
    <script async type="text/javascript">
        start();
    </script>
</html>
```

----

# 実装コード(script)

```javascript
window.start = async function() {
    const video = document.getElementById("myvideo");
    const stream = await navigator.mediaDevices.getUserMedia({
        video: {
            facingMode: 'user',
            width: 480,
            height: 320,
        },
        audio: false,
    });
    video.srcObject = stream;

    const [track] = stream.getVideoTracks();
    const {width, height} = track.getSettings();

    const button = document.getElementById("snap");
    button.addEventListener("click", e => {
        const canvas = document.createElement('canvas');
        canvas.setAttribute('width', width);
        canvas.setAttribute('height', height);
        const context = canvas.getContext('2d');
        context.drawImage(video, 0, 0, width, height);

        const dataUrl = canvas.toDataURL('image/png');
        const image = document.getElementById("destination");
        image.style.display = "inline";
        image.src = dataUrl;
    });
}
```

----

# 動作確認 #2

----

# 必要なものはできてきた！

- 画像変換をするプログラム
- ブラウザで画像を撮影し、データを取り出すJavaScript

----

# これらをつなぎ合わせる

TBA: 図

----

# RustのプログラムをWASM化しよう

----

# WASMに合わせてシグネチャを工夫する

- データをdataURL形式で取得したい
- 返却もbase64にエンコードしたい
  - そのままdataURL形式に流用
- base64の文字列なので **ゼロ終端** できる

----

# 上記方針で実装しよう

----

# 最初は一応PCで確認する


----

# WASMにコンパイルする

- imageのfeatureをpngに絞ったのが効いてくる

----

# JavaScript側の繋ぎを実装しよう

----

# ファイル一式をまとめよう

- また `web` ディレクトリを掘っておく

----

# 通しで動作確認...

----

- できましたのキャプチャ

----

<!--
_class: hero
-->

# まとめ

----

# 今日のまとめ

## 今までの中間成果として...

- ブラウザで取得した画像をWASMに渡し、加工してブラウザに戻し表示する、という一連を実装しました
- ブラウザとWASMの実際的な連携を体験できました
- データを渡す際の留意点も体感できたかと思います

----

<!--
_class: hero
-->

# 演習課題

----

# 演習課題

- **1)** `pixelate()` という関数名で、ピクセレーション（モザイク化）を実装しましょう。
  - Rust/WASMでのシグネチャはgrayscaleと同じはずですね！
  - 例えば 10*10 の区間に区切って左上のピクセルの色で埋める、という感じの実装が考えられます
    - 少し調べたり工夫してみよう
    - 一応、サンプルコードが本講義のリポジトリにあります

----

# 演習課題

- **2)** 今日実装した内容を `wasm-bindgen` を使って再実装してみましょう。
  - コードは減るのですが、色々と隠蔽されます

----

- WIP

----

# memo: 骨組み

- Rustだけで画像変換を実装
- ブラウザで「カメラ」を実装（blobとdataurl）
- 画像変換プログラムをWASMモジュールにする
- エラーにならないかを確認
- 変換した画像を線形メモリから取り出す
- 取り出した画像をブラウザdomに挿入する

----

# 次回

- #4 Scratch をハックする方法を学ぼう
  - <span style='font-size: 30pt'>予定: 2025/1/XX(YY) 14:00 start</span>
  - キーワード:
    - ScratchのMOD
    - React
    - TypeScript
- TBA: 2025年の予定...
