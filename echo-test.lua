-- Set global request parameters
wrk.method = "POST"
wrk.body   = '{"message":"hello"}'
wrk.headers["Content-Type"] = "application/json"

