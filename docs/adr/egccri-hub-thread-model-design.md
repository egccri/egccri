# egccri-hub thread model

### Date: 2021-07-08

## Status

In process

## Context

Message QoS and high performance.

## Decision

reactor thread model.

##### dataflow:

```text
        
messages -> QoS pipeline -> send pipeline ->
                                            |-> collect pipeline -> MQ
messages -> QoS pipeline -> send pipeline ->

```

## Consequences