const BlockType = require('../../extension-support/block-type');
const ArgumentType = require('../../extension-support/argument-type');
const StageLayering = require('../../engine/stage-layering');

const fukuokaIcon = require('./icon.png');

class Scratch3FukuokaBlocks {
    constructor(runtime) {
        this.runtime = runtime;

        this.captureSkinId = null;
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

        this.runtime.ioDevices.video.disableVideo();
        this.captureSkinId = this.runtime.renderer.createBitmapSkin(canvas, 1);
        const drawableId = this.runtime.renderer.createDrawable(
            StageLayering.BACKGROUND_LAYER
        );
        this.runtime.renderer.updateDrawableProperties(drawableId, {
            skinId: this.captureSkinId,
        });
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