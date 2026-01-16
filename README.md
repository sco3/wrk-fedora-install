# Installing wrk on Fedora

`wrk` is a HTTP benchmarking tool capable of generating significant load when run on a single multi-core CPU. 


## Prerequisites

Before building `wrk`, you need to install the necessary development tools and Perl dependencies required for the build scripts to run correctly on Fedora.

### 1. Install Development Tools
```bash
sudo dnf groupinstall "Development Tools"
sudo dnf install openssl-devel git
```

### 2. Test with curl

```
curl -X POST -H 'Content-Type:application/json' --data '{"message":"hello"}' http://0.0.0.0:9080/api/echo
```


### 3. Test with wrk
```lua
-- Set global request parameters
wrk.method = "POST"
wrk.body   = '{"message":"hello"}'
wrk.headers["Content-Type"] = "application/json"

```

the test command
```
wrk -t12 -c200 -d6s -s echo-test.lua --latency http://0.0.0.0:9080/api/echo
```
