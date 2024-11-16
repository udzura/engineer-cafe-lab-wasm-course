# 今日は少しWASMから離れます

# Scratch のカスタムブロックを作ってみる

# 環境構築

```
$ git clone --depth=1 https://github.com/scratchfoundation/scratch-vm.git
$ git clone --depth=1 https://github.com/scratchfoundation/scratch-gui.git
```

- scratch-guiはすごくでかいです。

----

# まずはローカルに立ち上げる

- 参考ページ https://ja.scratch-wiki.info/wiki/Scratch_3.0%E3%81%AE%E6%8B%A1%E5%BC%B5%E6%A9%9F%E8%83%BD%E3%82%92%E4%BD%9C%E3%81%A3%E3%81%A6%E3%81%BF%E3%82%88%E3%81%86/%E4%B8%8B%E6%BA%96%E5%82%99

## npm のインストール

- Macの場合
```
$ brew install npm
```

- Windowsの場合
  - WSLにnodejs、npmを入れる
```
$ sudo apt install -y nodejs npm
```
  - [nvm-windows](https://learn.microsoft.com/ja-jp/windows/dev-environment/javascript/nodejs-on-windows) でもいいかも？

----

## ライブラリインストールとリンク

```
$ cd scratch-vm
$ npm install
$ npm link

$ cd ../scratch-gui
$ npm install
$ npm link scratch-vm
```

----

## 起動させる

```
$ npm start
...
<i> [webpack-dev-server] Project is running at:
<i> [webpack-dev-server] Loopback: http://localhost:8601/, http://[::1]:8601/
<i> [webpack-dev-server] On Your Network (IPv4): http://192.168.64.1:8601/
<i> [webpack-dev-server] On Your Network (IPv6): http://[fd8b:9dc5:a15c:3ef3:104c:e908:2b4c:9a03]:8601/
<i> [webpack-dev-server] Content not from webpack is served from '/Users/udzura/ghq/github.com/scratchfoundation/scratch-gui/public' directory
...
```

- アドレスが先頭に出るので流されがち
- http://localhost:8601/ にアクセス！

----

![alt text](image.png)

----

# テストプラグインの追加

600x372と80x80の画像を用意する

![alt text](image-1.png)

![alt text](image-2.png)

----

## extension ディレクトリを作成

```
# in scratch-gui project
$ mkdir -p src/lib/libraries/extensions/fukuoka
$ cp banner.png icon.png src/lib/libraries/extensions/fukuoka
```

## extension 管理ファイルを編集

- `src/lib/libraries/extensions/index.jsx` を編集

```jsx
//...
import fukuokaIconURL from './fukuoka/banner.png'
import fukuokaInsetIconURL from './fukuoka/icon.png'

export default [ // この行の直後に追加
    {
        name: (
            <FormattedMessage
                defaultMessage="Hello ECF!"
                description="Engineer Cafe Fukuoka Sample Plugin"
                id="gui.extension.fukuoka.name"
            />
        ),
        extensionId: 'fukuoka',
        iconURL: fukuokaIconURL,
        insetIconURL: fukuokaInsetIconURL,
        description: (
            <FormattedMessage
                defaultMessage="Hello ECF!"
                description="Engineer Cafe Fukuoka Sample Plugin; This is test!"
                id="gui.extension.fukuoka.description"
            />
        ),
        featured: true
    },
//...
//...
```

----

## この状態でブラウザを再確認

- ホットリロードされるのでブラウザリロードだけでいい
- プラグイン一覧を見ると...!!!

![alt text](image-3.png)

----

# プラグインの中身を作る

- 次は `scratch-vm` プロジェクトに移動する（VSCodeで開くなら開く）
- `src/extensions/scratch3_fukuoka` を作成
- `src/extensions/scratch3_fukuoka/icon.png` をコピー
- `src/extensions/scratch3_fukuoka/index.js` を作る
  - `jsx` ではない！

----

```javascript
const BlockType = require('../../extension-support/block-type');
const fukuokaIcon = require('./icon.png');

class Scratch3FukuokaBlocks {
    constructor (runtime) {
        this.runtime = runtime;
    }

    getInfo () {
        return {
            id: 'fukuoka',
            name: 'Hello ECF',
            menuIconURI: fukuokaIcon,
            blockIconURI: fukuokaIcon,
            blocks: [
                {
                    opcode: 'testAlert',
                    blockType: BlockType.COMMAND,
                    text: 'alert something',
                    arguments: {},
                },
            ],
            menus: {},
        };        
    }
    testAlert() {
        alert("Hello ECF!!");
    }
}
module.exports = Scratch3FukuokaBlocks;
```

----

## FYI: BlockTypeの話

| 定数 | 何タイプ？ | 使い方 | 
| ---- | ---- | ---- | 
| `BlockType.BOOLEAN` | 真偽ブロック |  |
| `BlockType.COMMAND` | スタックブロック/コマンドブロック |  |
| `BlockType.HAT` | ハットブロック |  |
| `BlockType.LOOP` | C型ブロック |  |
| `BlockType.REPORTER` | 値ブロック |  |

----

## プラグインとしてロード

- `src/extension-support/extension-manager.js` を編集

```javascript
//...
const builtinExtensions = {
    // ...
    coreExample: () => require('../blocks/scratch3_core_example'),
    // These are the non-core built-in extensions. ....

    // 最後に追加
    fukuoka: () => require('../extensions/scratch3_fukuoka'),
};
//...
```

----

# 動作確認

- `scratch-gui` に戻って `npm start` コマンドを再起動
- 「拡張機能を選ぶ」からHello ECF!を選択すると...

![alt text](image-4.png)

----

## 他のブロックとも組み合わせ可能

![alt text](image-5.png)

----

# もう少し踏み込んだ機能を使ってみよう

----

# ビデオを制御してみよう

- 「ビデオモーションセンサー」のコードを参考にできる
- まず、単純にオンオフするには？

----

## ブロックの定義を編集する

```javascript
class Scratch3FukuokaBlocks {
    constructor(runtime) {
        this.runtime = runtime;
    }

    _VIDEO_STATE_ITEMS() {
        return [
            {text: 'off', value: 'off'},
            {value: 'on', value: 'on'},
        ];
    }
    
    getInfo() {
        return {
            id: 'fukuoka',
            name: 'Video Capture',
            menuIconURI: fukuokaIcon,
            blockIconURI: fukuokaIcon,
            blocks: [
                {
                    opcode: 'videoToggle',
                    blockType: BlockType.COMMAND,
                    text: 'turn video [VIDEO_STATE]',
                    arguments: {
                        VIDEO_STATE: {
                            type: ArgumentType.NUMBER,
                            menu: 'VIDEO_STATE',
                            defaultValue: 'on',
                        }
                    }
                }
            ],
            menus: {
                VIDEO_STATE: {
                    acceptReporters: true,
                    items: this._VIDEO_STATE_ITEMS(),
                }
            },
        };        
    }

    videoToggle(args) {
        const state = args.VIDEO_STATE;
        if (state === 'off') {
            this.runtime.ioDevices.video.disableVideo();
        } else {
            this.runtime.ioDevices.video.enableVideo();
            this.runtime.ioDevices.video.mirror = true;
        }
    }
}
```

----

## 再読み込みすればビデオon/offが動いている

![alt text](image-6.png)

----

## ビデオの画像をキャプチャしたい

```javascript
    getFrame() {
        const imageData = this.runtime.ioDevices.video.getFrame({
            format: 'image-data',
        });
        const canvas = document.createElement('canvas');
        canvas.width = imageData.width;
        canvas.height = imageData.height;
        canvas.getContext('2d').putImageData(imageData, 0, 0);
        console.log(canvas.toDataURL());
        // TODO: ダウンロードさせたいけど...
    }
```

## scratchの画面のどこかに表示したい

```javascript
    getFrameAndSnapshot() {
        const imageData = this.runtime.ioDevices.video.getFrame({
            format: 'image-data',
        });
        const canvas = document.createElement('canvas');
        canvas.width = imageData.width;
        canvas.height = imageData.height;
        canvas.getContext('2d').putImageData(imageData, 0, 0);
        console.log(canvas.toDataURL());

        this.runtime.ioDevices.video.disableVideo();
        this.captureSkinId = this.runtime.renderer.createBitmapSkin(canvas, 1);
        const drawableId = this.runtime.renderer.createDrawable(
            StageLayering.BACKGROUND_LAYER
        );
        this.runtime.renderer.updateDrawableProperties(drawableId, {
            skinId: this.captureSkinId,
        });
    }
```

----

# ちなみに... Webで公開したい時

```
$ npm run build
```

- `build/` にstaticなファイル一式ができる
- static fileとして表示確認するには以下

```
$ python3 -m http.server 8080
## http://127.0.0.1:8080/build/ を開く
```

- あとはgithub pagesなどでそのまま公開すればOK
