# もっと複雑なことをしたい！

# 画像を処理


# 今日の学習の流れ

- Rustで画像処理の基本的なコードを書く
  - グレースケール化
- それをWASMにする
- また、ブラウザでカメラめいたものを実装する
- そして... それらのデータを繋ぎこむ！

# やることが... 多い！

- 頑張りましょう。年末だし
- 今日間に合わなくても、Discordや別日での質問を活用しよう
（私も今年中何度かエンジニアカフェに出勤します！）

# Rustで画像処理

# 画像のグレースケール化

# 画像の「明るさ」の基本概念

- 色によって感じ方が違う
- 人間の明るさの感じ方は線形でない

# ガンマ補正とは？

- 人間の明るさの感じ方は線形でない
- なので、強さによって自然に見えるよう、ある程度ディスプレイでの出し方の補正をかけている
- これがガンマ補正

# なのでグレースケールにするときも...

- 「色によって感じ方が違う」3色をまとめるのでガンマ補正を考慮しないといけない
  - モニタでは3色それぞれに曲線がある

# 実際やってみよう

- 「平均値」で画像をグレースケールにする

※ この資料/サンプルコードの中で、画像のピクセル化の実装例も提示します。

# Rustでの実装

- imageクレートを使う
  - なるべく低レイヤと言ったが...
  - 流石にPNGのフォーマットをパースして、までは時間がないため...
- このクレートはそのままWASMでも動く！

# 実装コード

```rust
```

# 動作確認

# ブラウザで「カメラ」を実装する

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

# 動作確認 #2



# memo: 骨組み

- Rustだけで画像変換を実装
- ブラウザで「カメラ」を実装（blobとdataurl）
- 画像変換プログラムをWASMモジュールにする
- エラーにならないかを確認
- 変換した画像を線形メモリから取り出す
- 取り出した画像をブラウザdomに挿入する
