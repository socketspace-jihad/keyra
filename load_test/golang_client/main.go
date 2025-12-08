package main

import (
	"encoding/binary"
	"fmt"
	"io"
	"net"
	"sync"
	"sync/atomic"
	"time"
)

const (
	TargetAddr   = "127.0.0.1:4000"
	NumWorkers   = 64
	BatchSize    = 10000
	MsgPerWorker = 10000000
)

const ResponseSize = 2

func makePayload() []byte {
	op := 1
	key := []byte("name")
	val := []byte("jihad")
	buf := make([]byte, 0, 4+len(key)+len(val))
	buf = append(buf, byte(op))
	buf = append(buf, byte(len(key)))
	valLenBytes := make([]byte, 2)
	binary.BigEndian.PutUint16(valLenBytes, uint16(len(val)))
	buf = append(buf, valLenBytes...)
	buf = append(buf, key...)
	buf = append(buf, val...)
	return buf
}

func connectWithRetry() *net.TCPConn {
	for {
		conn, err := net.DialTimeout("tcp", TargetAddr, 5*time.Second)
		if err != nil {
			time.Sleep(100 * time.Millisecond)
			continue
		}

		tcpConn := conn.(*net.TCPConn)
		tcpConn.SetNoDelay(true)
		tcpConn.SetKeepAlive(true)
		tcpConn.SetReadBuffer(1024 * 1024) // 1460 for avoiding fragmentation ?
		tcpConn.SetWriteBuffer(1024 * 1024)

		return tcpConn
	}
}

func main() {
	var ops uint64
	var totalLatencyMicro uint64
	var failed uint64

	basePayload := makePayload()
	payloadLen := len(basePayload)
	totalReqs := uint64(NumWorkers * MsgPerWorker)

	fmt.Printf("Target: %s | Workers: %d | BatchSize: %d | Total Req: %d\n",
		TargetAddr, NumWorkers, BatchSize, totalReqs)

	startTest := time.Now()

	go func() {
		var lastOps uint64
		ticker := time.NewTicker(1 * time.Second)
		defer ticker.Stop()
		for range ticker.C {
			curr := atomic.LoadUint64(&ops)
			if curr >= totalReqs {
				return
			}
			diff := curr - lastOps
			lastOps = curr
			fmt.Printf("Throughput: %d req/sec\n", diff)
		}
	}()

	var wg sync.WaitGroup
	wg.Add(NumWorkers)

	for i := 0; i < NumWorkers; i++ {
		go func() {
			defer wg.Done()

			batchWriteBuf := make([]byte, BatchSize*payloadLen)
			for k := 0; k < BatchSize; k++ {
				copy(batchWriteBuf[k*payloadLen:], basePayload)
			}

			batchReadBuf := make([]byte, BatchSize*ResponseSize)

			conn := connectWithRetry()
			defer conn.Close()

			requestsLeft := MsgPerWorker

			for requestsLeft > 0 {
				currentBatch := BatchSize
				if requestsLeft < BatchSize {
					currentBatch = requestsLeft
				}

				currentWriteBuf := batchWriteBuf[:currentBatch*payloadLen]
				currentReadBuf := batchReadBuf[:currentBatch*ResponseSize]

				tStart := time.Now()

				_, err := conn.Write(currentWriteBuf)
				if err != nil {
					conn.Close()
					atomic.AddUint64(&failed, uint64(currentBatch))
					conn = connectWithRetry()
					continue
				}

				_, err = io.ReadFull(conn, currentReadBuf)
				if err != nil {
					conn.Close()
					atomic.AddUint64(&failed, uint64(currentBatch))
					conn = connectWithRetry()
					continue
				}

				latency := time.Since(tStart).Microseconds()
				atomic.AddUint64(&totalLatencyMicro, uint64(latency))
				atomic.AddUint64(&ops, uint64(currentBatch))

				requestsLeft -= currentBatch
			}
		}()
	}

	wg.Wait()
	duration := time.Since(startTest)
	success := atomic.LoadUint64(&ops)

	fmt.Println("\n--- DONE ---")
	fmt.Printf("Duration: %v\n", duration)
	fmt.Printf("Throughput: %.2f req/sec\n", float64(success)/duration.Seconds())
	if success > 0 {
		fmt.Printf("Avg Latency (Batch) divided by batch size: %.2f µs\n", float64(atomic.LoadUint64(&totalLatencyMicro))/float64(success/uint64(BatchSize))/BatchSize)
	}
}
