const BlockType = require('../../extension-support/block-type');
const ArgumentType = require('../../extension-support/argument-type');
const StageLayering = require('../../engine/stage-layering');

const log = require('../../util/log');

const fukuokaIcon = require('./icon.png');

class Scratch3FukuokaBlocks {
    constructor(runtime) {
        this.runtime = runtime;

        this.captureSkinId = null;

        this.wasm = null;

        const obj = {}
        WebAssembly.instantiateStreaming(fetch("/static/wasm/fukuoka_ecl_imagetest.wasm"), obj).then(
            (wasm) => {
                this.wasm = wasm.instance;
                wasm.instance.exports.memory.grow(10);
                log.debug("wasm load OK")
            },
        );

        this._workerRunning = false;
        this._workerMessage = null

        this.worker = new Worker("/static/workers/fib_worker.js");
        this.worker.onmessage = (e) => {
            this._workerMessage = e.data[0]
        }
    }

    _VIDEO_STATE_ITEMS() {
        return [
            {text: 'off', value: 'off'},
            {text: 'on', value: 'on'},
        ];
    }
    
    getInfo() {
        return {
            id: 'fukuoka',
            name: 'Fib calc',
            menuIconURI: fukuokaIcon,
            blockIconURI: fukuokaIcon,
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
                {
                    opcode: 'videoToggle',
                    blockType: BlockType.COMMAND,
                    text: 'turn video [VIDEO_STATE]',
                    arguments: {
                        VIDEO_STATE: {
                            type: ArgumentType.STRING,
                            menu: 'VIDEO_STATE',
                            defaultValue: 'on',
                        }
                    }
                },
                {
                    opcode: 'getFrame',
                    blockType: BlockType.COMMAND,
                    text: 'get current video image',
                    arguments: {}
                },
            ],
            menus: {
                VIDEO_STATE: {
                    acceptReporters: true,
                    items: this._VIDEO_STATE_ITEMS(),
                }
            },
        };
    }

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

    videoToggle(args) {
        const state = args.VIDEO_STATE;
        if (state === 'off') {
            this.runtime.ioDevices.video.disableVideo();
            if (this.captureSkinId !== null) {
                this.runtime.renderer.destroySkin(this.captureSkinId);
                this.captureSkinId = null;
            }
        } else {
            this.runtime.ioDevices.video.enableVideo();
            this.runtime.ioDevices.video.mirror = true;
        }
    }

    getFrame() {
        const imageData = this.runtime.ioDevices.video.getFrame({
            format: 'image-data',
        });
        const canvas = document.createElement('canvas');
        canvas.width = imageData.width;
        canvas.height = imageData.height;
        canvas.getContext('2d').putImageData(imageData, 0, 0);

        // let x = window.open("", "_blank", `width=${canvas.width},height=${canvas.height}`);
        // x.document.open();
        // x.document.appendChild(canvas);
        if (!this.wasm) {
            return;
        }

        const offset = this.wasm.exports.__heap_base;
        const memory = this.wasm.exports.memory;
        const dataUrl = canvas.toDataURL("image/png");
        let buffer = new Uint8Array(memory.buffer, offset, dataUrl.length);
        for( var i = 0; i < dataUrl.length; i++ ) {
            buffer[i] = dataUrl.charCodeAt(i);
        }
        const resOffset = this.wasm.exports.grayscale_blob(canvas.width, canvas.height, offset, dataUrl.length);
        log.debug("wasm run OK")

        let resultBuf = new Uint8Array(memory.buffer, resOffset, 1 << 22);
        let resultBase64 = "";
        for ( var i = 0; resultBuf[i] != 0; i++) {
            resultBase64 += String.fromCharCode(resultBuf[i]);
        }
        log.debug("debug image url:")
        console.log("data:image/png;base64," + resultBase64);

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
    }

    // getInfo () {
    //     return {
    //         id: 'fukuoka',
    //         name: 'Hello ECF',
    //         menuIconURI: fukuokaIcon,
    //         blockIconURI: fukuokaIcon,
    //         blocks: [
    //             {
    //                 opcode: 'testAlert',
    //                 blockType: BlockType.COMMAND,
    //                 text: 'alert something',
    //                 arguments: {},
    //             },
    //         ],
    //         menus: {},
    //     };        
    // }

    // testAlert() {
    //     alert("Hello ECF!!");
    // }
}

module.exports = Scratch3FukuokaBlocks;