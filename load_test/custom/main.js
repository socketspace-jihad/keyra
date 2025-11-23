const net = require('net');

const TOTAL_REQUESTS = 1000;
const TEST_DURATION = 30 * 1000; // 30 detik
let latencies = [];

function createSetCommand(key, val) {
    const key_len = key.length;
    const val_len = val.length;

    const buf = Buffer.alloc(1 + 1 + 2 + key_len + val_len);
    let offset = 0;
    buf.writeUInt8(0x01, offset); offset += 1;
    buf.writeUInt8(key_len, offset); offset += 1;
    buf.writeUInt16BE(val_len, offset); offset += 2; // big endian

    // Copy key
    buf.write(key, offset, key_len, 'utf8');
    offset += key_len;

    // Copy value
    buf.write(val, offset, val_len, 'utf8');

    return buf;
}

function sendTcpRequest(key, val) {
    return new Promise((resolve, reject) => {
        const client = new net.Socket();
        const buf = createSetCommand(key, val);
        const start = Date.now();

        client.connect(4000, '127.0.0.1', () => {
            client.write(buf);
        });

        client.on('data', (data) => {
            const latency = Date.now() - start;
            latencies.push(latency);
            client.destroy(); // close connection
            resolve(latency);
        });

        client.on('error', (err) => {
            client.destroy();
            reject(err);
        });
    });
}

async function runTest() {
    console.log("running the test");
    const endTime = Date.now() + TEST_DURATION;
    while (Date.now() < endTime) {
        const promises = [];
        for (let i = 0; i < TOTAL_REQUESTS; i++) {
            promises.push(sendTcpRequest('name', 'jihad').catch(() => 0));
        }
        await Promise.all(promises);
    }

    const sum = latencies.reduce((a, b) => a + b, 0);
    const avg = sum / latencies.length;
    console.log(`Total requests: ${latencies.length}`);
    console.log(`Average latency: ${avg.toFixed(2)} ms`);
}

runTest().catch(console.error);

