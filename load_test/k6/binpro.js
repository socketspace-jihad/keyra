import tcp from 'k6/x/tcp';
import { check, sleep } from 'k6';
import { Trend } from 'k6/metrics';

export let options = {
    vus: 50,
    duration: '30s',
};

let tcp_latency = new Trend('tcp_latency');

export default function () {
    // 1. Connect ke TCP server
    const conn = tcp.connect('127.0.0.1:4000');
    check(conn, { 'tcp connected': (c) => c !== null });

    // 2. Siapkan SET command
    const key = 'name';
    const val = 'jihad';
    const key_len = key.length;
    const val_len = val.length;

    // Format: 0x01 | key_len(1) | val_len(2) | key | val
    const buf = new ArrayBuffer(1 + 1 + 2 + key_len + val_len);
    const view = new DataView(buf);
    let offset = 0;
    view.setUint8(offset, 0x01); offset += 1;
    view.setUint8(offset, key_len); offset += 1;
    view.setUint16(offset, val_len, false); offset += 2; // big endian

    // Copy key
    for (let i = 0; i < key_len; i++) view.setUint8(offset + i, key.charCodeAt(i));
    offset += key_len;

    // Copy value
    for (let i = 0; i < val_len; i++) view.setUint8(offset + i, val.charCodeAt(i));

    // 3. Kirim command
    const start = new Date().getTime();
    const resp = conn.write(buf);

    // 4. Baca response
    const respBuf = conn.read(1); // expect 1 byte
    tcp_latency.add(new Date.now() - start);

    check(respBuf, { 'tcp ok': (r) => r && r[0] === 0x00 });

    conn.close();
    sleep(0.01);
}

