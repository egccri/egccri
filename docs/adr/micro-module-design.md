# Micro module design

### Date: 2022-12-21

## Status

Pre commit

## Context

We need a lightweight async module for split module.

## Decision

+ 多个模块之间可以互相通信，通信方式为消息，不可以为传递结构体，因为要考虑拆分
+ 考虑每个模块传入AsyncRuntime或者每个module新建Runtime
+ 考虑standalone拆分为分布式
+ 模块要提供阻塞线程和非组塞线程

## Consequences

模块既可以传入公共Runtime，又可以新建，传入要检查线程数是否够