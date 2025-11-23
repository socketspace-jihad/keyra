import grpc from "k6/net/grpc";
import http from "k6/http";
import { check } from "k6";
import redis from 'k6/experimental/redis';


import { Trend } from "k6/metrics";
const redisLatency = new Trend("redis_latency", true);
const keyraGrpcLatency = new Trend("keyra_grpc_latency",true);

const client = new grpc.Client();
client.load(["."], "../../schema/grpc/data.proto");

let connected = false;

const redisClient = new redis.Client("redis://localhost:6379");

export default function () {
    if(!connected)  {
      client.connect("localhost:50051", { plaintext: true });
      connected = true;
    }
    let start = Date.now();
    let res2 = client.invoke("storage.KeyValue/Set", {
        key: "name",
        value: { stringVal: "jihad" },
    });
    keyraGrpcLatency.add(Date.now()-start);
    check(res2, { "grpc ok": (r) => r.status === grpc.StatusOK });
    // HTTP SET
    // let res1 = http.post("http://localhost:8081/set", JSON.stringify({
    //    key: "name",
    //    value: {
     //     "STRING":"jihad"
      //  }
   // }),{headers:{"Content-Type":"application/json"}})
    //check(res1, { "http ok": (r) => r.status === 200 });

  try {
        let start2 = Date.now();
        redisClient.set("name", "jihad");
        redisLatency.add(Date.now()-start2);
        check(true, { "redis ok": () => true });
    } catch (e) {
        check(false, { "redis ok": () => false });
    }

}

