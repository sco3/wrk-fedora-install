#!/usr/bin/env -S bash

wrk -t12 -c200 -d6s -s echo-test.lua --latency http://0.0.0.0:9080/api/echo