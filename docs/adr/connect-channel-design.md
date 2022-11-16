# 消息流转管道设计

### Date: 2022-11-16

## Status

Pre commit

## Context

消息通过设备上报到egccri，要对消息根据topic进行隔离，发送到不同的channel

考虑topic通配符

考虑按设备进行配置或者按产品分割

消息在流转中要可以分叉，计算