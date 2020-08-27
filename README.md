# rust146hang

Increasingly slow compilation as more levels of `async` are added

This is a somewhat minimal testcase extracted from a crate I'm working on. I was using Rust 1.45 and it compiled in a reasonable amount of time, then I upgraded to Rust 1.46 and suddenly compilation seemed to hang forever. Based on this more minimal test case I'm guessing it's not actually hanging forever, just taking a very long time.

I've found that the slowness seems tied to the depth of the async call chain. In the example code I have a long call chain: `handle_req_1` is called by `handle_req_2` is called by `handle_req_3`, etc. Here's the compilation timing I've observed when changing `handle_req_final` to directly call one of the `handle_req_N` functions:

handle_req_1 -> 0m12s
handle_req_2 -> 0m28s
handle_req_3 -> 1m06s
handle_req_4 -> 2m18s
handle_req_5 -> 5m01s

(Caveat: unscientific timings, these are not averaged over multiple
runs or anything.)
