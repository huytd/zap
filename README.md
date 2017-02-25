# zap

HTTP web server using [tokio-minihttp](https://github.com/tokio-rs/tokio-minihttp)

**Benchmark:**

Debug mode:

```
➜ wrk -t12 -c400 -d30s http://127.0.0.1:8080/

Running 30s test @ http://127.0.0.1:8080/
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    65.42ms   22.33ms 215.77ms   67.65%
    Req/Sec   498.93     96.53   740.00     66.11%
  178964 requests in 30.09s, 23.38MB read
  Socket errors: connect 0, read 206, write 0, timeout 0
Requests/sec:   5946.76
Transfer/sec:    795.61KB
```

Release mode:

```
➜ wrk -t12 -c400 -d30s http://127.0.0.1:8080/

Running 30s test @ http://127.0.0.1:8080/
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     8.39ms    5.74ms  75.83ms   77.64%
    Req/Sec     4.09k     0.91k   15.81k    76.92%
  1468048 requests in 30.07s, 191.81MB read
  Socket errors: connect 0, read 230, write 0, timeout 0
Requests/sec:  48817.30
Transfer/sec:      6.38MB
```
