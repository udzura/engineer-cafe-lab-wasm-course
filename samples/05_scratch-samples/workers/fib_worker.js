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
        postMessage([result]);
    }
}