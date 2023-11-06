# Parallelism

Sahara operates under the [actor model](https://en.wikipedia.org/wiki/Actor_model), with each [execution
context](./execution-context.md) acting as an individual actor. Contexts can communicate with one another only via
message passing and cannot generally share state in other ways (though there are limited mechanisms available via the
[global heap](./dynamic-memory.md#the-global-heap)]).

## Context allocation

## Message passing

## Scheduling