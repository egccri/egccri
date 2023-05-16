# storage module design

### Date: 2023-05-16

## Status

Pre commit

## Context

在`Egccri`中存在两个地方需要用到Wasm运行时，一个是流计算，一个是普通应用。在流计算中，
运行时充当一个算子，包裹在一个DAG的节点中，节点从管道中接收数据，交给WASM模块计算，得到
结果后再通过节点交给下游的Connector；而在普通应用中，WASM充当一个Serverless的应用，
通过SDK与`Egccri`环境中的各种组件交互，以及使用主机环境的各种能力。

但这样无疑增加了系统以及产品的复杂性。

## Decision

在流计算中同样使用SDk方式。
