# #5 Scratch とWebAssemblyを連携させよう

# いよいよ総まとめ！

# 前回の復習も兼ねて

- #4 の演習問題の解説を行います
  - （時間によりますが）

# 環境は前回のものをそのまま使用しましょう

----

# WASMバイナリをプロジェクトにコピー

- 第3回で作ったgrayscaleのWASMプログラムをそのまま使う
- scratch-guiプロジェクトの `static/wasm/grayscale.wasm` にコピーする

----

# プラグインの実装変更

- constructorから変更

```javascript
class Scratch3FukuokaBlocks {
    constructor(runtime) {
        this.runtime = runtime;

        this.captureSkinId = null;

        this.wasm = null;

        const obj = {}
        WebAssembly.instantiateStreaming(fetch("/static/wasm/grayscale.wasm"), obj).then(
            (wasm) => {
                this.wasm = wasm.instance;
                wasm.instance.exports.memory.grow(10);
                log.debug("wasm load OK")
            },
        );
    }
//...
```

----

## operation関数本体を実装

```javascript
    snapshotVideo() {
        const imageData = this.runtime.ioDevices.video.getFrame({
            format: 'image-data',
        });
        const canvas = document.createElement('canvas');
        canvas.width = imageData.width;
        canvas.height = imageData.height;
        canvas.getContext('2d').putImageData(imageData, 0, 0);

        if (!this.wasm) {
            return;
        }
```

----

```javascript
        const offset = this.wasm.exports.__heap_base;
        const memory = this.wasm.exports.memory;
        const dataUrl = canvas.toDataURL("image/png");
        let buffer = new Uint8Array(memory.buffer, offset, dataUrl.length);
        for( var i = 0; i < dataUrl.length; i++ ) {
            buffer[i] = dataUrl.charCodeAt(i);
        }
        const resOffset = this.wasm.exports.grayscale(
            canvas.width, canvas.height, offset, dataUrl.length);
        log.debug("wasm run OK")
```

----

```javascript
        let resultBuf = new Uint8Array(memory.buffer, resOffset, 1 << 22);
        let resultBase64 = "";
        for ( var i = 0; resultBuf[i] != 0; i++) {
            resultBase64 += String.fromCharCode(resultBuf[i]);
        }
        log.debug("debug image url:")
        console.log("data:image/png;base64," + resultBase64);
        // 一旦ここまで
    }
```

----

# ここまでで動作確認しよう

```javascript
    getInfo() {
        return {
            id: 'fukuoka',
            name: 'Video Capture',
            menuIconURI: fukuokaIcon,
            blockIconURI: fukuokaIcon,
            blocks: [ //...
                { // 最後に
                    opcode: 'snapshotVideo',
                    blockType: BlockType.COMMAND,
                    text: 'snapshot current video',
                    arguments: {}
                },
            ],
            menus: { ... },
        };
    }
```

----

## DataURL ができた

TODO: capture

----

# dataURLをScratchのスキンに差し込む

- 画像データに変換するには、一旦 `Image` オブジェクトに差し込む

```javascript
const image = new Image()
image.src = "data:image/png;base64," + resultBase64;
```

- しかし、その直後 `drawImage()` を呼んでもうまくいかない

```javascript
const image = new Image()
image.src = "data:image/png;base64," + resultBase64;
canvas.getContext('2d').drawImage(image, 0, 0, canvas.width, canvas.height);
// => canvasが更新されない
```

----

## 実は画像のソース読み込みは非同期処理

- `image.src = ...` で更新した直後では描画がされていない
- どうするか？ `image.onload` を利用

----

```javascript
        // ....
        const image = new Image()
        image.onload = () => {
            canvas.getContext('2d').clearRect(0, 0, canvas.width, canvas.height);
            canvas.getContext('2d').drawImage(image, 0, 0, canvas.width, canvas.height);

            this.runtime.ioDevices.video.disableVideo();
            this.captureSkinId = this.runtime.renderer.createBitmapSkin(canvas, 1);
            const drawableId = this.runtime.renderer.createDrawable(
                StageLayering.BACKGROUND_LAYER
            );
            this.runtime.renderer.updateDrawableProperties(drawableId, {
                skinId: this.captureSkinId,
            });
        }
        image.src = "data:image/png;base64," + resultBase64;
    } // snapshotVideo end
```

----

# あとはこれを呼び出すようにすればOK

- 諸々更新しての動作確認:

----

# 全て完成！！

- Scratch のブロックからWebAssemblyの関数を呼び出し、画像加工をすることができた！

----

<!--
_class: hero
-->

# 〜 完 〜

----

<!--
_class: hero
-->

# 最終演習課題...

----

# 力ある人のため最終演習課題...

- 第5回の内容は、今までの高難度を反映して分量を少なくしています
- ですが、ここまでの内容も余裕を持ってこなしてきた方のために最後の課題を出します

----

# fibで始まり、fibに帰る

- 第1回で作ったfibモジュールを `static/wasm` にコピーします

```
$ cp path/tp/hello_wasm.wasm static/wasm/fib.wasm
```

- このfibをscratchのブロックにしてみましょう

----

# 実装例

```javascript
    constructor(runtime) {
        this.runtime = runtime;
        this.fib = null;

        const obj2 = {}
        WebAssembly.instantiateStreaming(fetch("/static/wasm/fib.wasm"), obj2).then(
            (wasm) => {
                this.fib = wasm.instance;
                log.debug("fib load OK")
            },
        );
    }

    getInfo() {
        return {
            id: 'fukuoka',
            name: 'Fib calc',
            menuIconURI: fukuokaIcon,
            blockIconURI: fukuokaIcon,
            blocks: [
                {
                    opcode: 'reportFib',
                    blockType: BlockType.REPORTER,
                    text: 'get fib of [N]',
                    arguments: {
                        N: {
                            type: ArgumentType.NUMBER,
                            defaultValue: 1,
                        }
                    }
                },
            ],
            menus: {},
        };
    }

    reportFib(args) {
        const n = args.N;
        const result = this.fib.exports.fib(n)
        log.debug(`answer: fib(${n}) = ${result}`)
        return result
    }

```

----

# これは動く

- このReporterはこんな感じで使えばいい


----

# しかし？

- ある程度以上大きなfib(N)を求めようとすると...？

----

# ブラウザはシングルスレッド

- WASMは（なんとなく勘違いしがちだが）クライアントで動く
- したがって、WASMで重い処理をするとクライアントのCPUは使い果たされます

----

# じゃあどうするか？

----

# WebWorker

- 色々やり方はありそうだが今回、WebWorkerに挑戦しよう

----

# WebWorker とは

- ブラウザでの一部の処理をバックグラウンドで動かすための仕組み
- フロントのメインスレッドを使い切らない
  - したがって描画や制御の挙動に影響しないようになる

----

# WebWorker でWASMを動かす

- **`scratch-gui`** 側に `static/workers/fib_worker.js` を作成
  - WebWorker は別のJavaScriptファイルを読み込むため、今回はscratch-guiにstatic fileを作っている
  - もっといいやり方があるかも

----

# `fib_worker.js` の編集

```javascript
this.wasm = null;
obj = {};
WebAssembly.instantiateStreaming(fetch("/static/wasm/fib.wasm"), obj).then(
    (wasm) => {
        this.wasm = wasm.instance;
        console.log("Worker: wasm load OK")
    },
);

onmessage = (e) => {
    console.log("Worker: Message received from main script");
    if (!this.wasm) {
        console.log("Worker: failed...");
        return;
    }
    const n = e.data[0]
    if (isNaN(n)) {
        console.log("Worker: failed...");
        return;
    } else {
        // console.log(this.wasm);
        const result = this.wasm.exports.fib(n);
        console.log('Worker: Posting message back to main script');
        console.log(`Worker: answer = ${result}`);
        // postMessage(result.toString());
    }
}
```

----

# Scratchからの呼び出し側の編集

```javascript
```

----

# Fibは成功はしていそう

- 正しい数がコンソールに表示されること
- 大きな数を与えても、メインスレッドがブロックしないこと
  - GUI操作が普通にできていること

## ということで、思い処理でもUIがブロックしなくなった

----

# ...あれ？

- 計算した値ってどうやって受け取るん？

----

# WebWorkerで値を受け取る

- worker処理側
  - `postMessage()` するとメインにあるworker側に返却できる
- メインスレッド側
  - workerインスタンスに `worker.onmessage` ハンドラがある
  - そこで受け取った値を処理する
  - イベント駆動

----

# Scratchでイベント駆動をするには...

- `BlockType.HAT` を使う！

----

## 下準備

- constructorにインスタンス変数を用意する

```javascript
    constructor(runtime) {
        this.runtime = runtime;
        this._workerRunning = false;
        this._workerMessage = null

        this.worker = new Worker("/static/workers/fib_worker.js");
    }
```

----

## ブロックの定義

```javascript
    blocks: [
        {
            opcode: 'startCalcFib',
            blockType: BlockType.COMMAND,
            text: 'start worker fib of [N]',
            arguments: {
                N: {
                    type: ArgumentType.NUMBER,
                    defaultValue: 1,
                }
            }
        },
        {
            opcode: 'whenFinishedFib',
            blockType: BlockType.HAT,
            text: 'when fib() is finished',
        },
        {
            opcode: 'reportFib',
            blockType: BlockType.REPORTER,
            text: 'answer of fib',
        },
    ] //...
```

----

## opハンドラの定義

```javascript
    startCalcFib(args) {
        const n = args.N;
        if (this._workerRunning) {
            return;
        }
        this._workerRunning = true;
        this._workerMessage = null;
        this.worker.postMessage([n]);
    }

    whenFinishedFib(args) {
        if (this._workerRunning && this._workerMessage !== null) {
            this._workerRunning = false
            return true;
        }
    }

    reportFib(args) {
        if (this._workerMessage === null) {
            return -1;
        } else {
            return this._workerMessage;
        }
    }
```

----

## イベントのライフサイクルを作図してみた

TODO: 頑張る

----

## constructorに戻って

- `onmessage` イベントを受け取って変数を更新するように変更

```javascript
    constructor(runtime) {
        this._workerRunning = false;
        this._workerMessage = null

        this.worker = new Worker("/static/workers/fib_worker.js");
        // ここを追加！
        this.worker.onmessage = (e) => {
            this._workerMessage = e.data[0]
        }
    }
```

----

## 最後にworker

- 最後に `postMessage` させる

```javascript
onmessage = (e) => {
    if (isNaN(n)) {
    // ...
    } else {
        const result = this.wasm.exports.fib(n);
        console.log('Worker: Posting message back to main script');
        console.log(`Worker: answer = ${result}`);
        postMessage([result]); // ここを追加
    }
}
```

----

# 最後の動作確認！


----

## 簡単な計算

----

## 重い処理は...

----

# より複雑なブロックも作れた

----

<!--
_class: hero
-->

# まとめ

----

# 今日のまとめ

## WebAssemblyとScratchと、友達になれたかな？

- ScratchからWASMを呼ぶ方法
  - 2パターン
- WebWorkerについて
- `BlockType.HAT` によるScratchのイベント駆動化

----

# 最後のはずなのに追加の演習課題

- **1)** WebWorkerのエラーを、メインスレッドで受け取れるようにするにはどうすればいいでしょうか？
- **2)** WebWorkerのエラーをScratchのブロック側で制御できるようにしたいです。
  - どう言う設計をして、どう言うブロックを用意するといいでしょうか？

// WASM でなくなってる、と言うツッコミは置いといて...

----

<!--
_class: hero
-->

# 本当におしまい

- 最後までありがとうございました

----

# 参考資料

- MDN: WebWorker
  - https://developer.mozilla.org/ja/docs/Web/API/Web_Workers_API
- WebWorker 周りの良い記事
  - [Webpack での設定例](https://qiita.com/ichitose/items/77ef8df1867017e9c483)
  - [React との連携方法](https://qiita.com/ichitose/items/77ef8df1867017e9c483)