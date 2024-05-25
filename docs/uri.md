# URI syntax

**Redis Standalone**

redis :// [username?: password@] host [: port] [/ database][? [timeout=timeout[d|h|m|s|ms|us|ns]] [&database=database]]

**Redis Standalone (SSL)**

rediss :// [username?: password@] host [: port] [/ database][? [timeout=timeout[d|h|m|s|ms|us|ns]] [&database=database]]

**Redis Standalone (Unix Domain Sockets)**

redis-socket :// path [?[timeout=timeout[d|h|m|s|ms|us|ns]][&database=database]]

**Redis Sentinel**

redis-sentinel :// [username?: password@] host1[: port1] [, host2[: port2]] [, hostN[: portN]] [/ database][?[timeout=timeout[d|h|m|s|ms|us|ns]] [&sentinelMasterId=sentinelMasterId] [&database=database]]

**Schemes**

+ `redis` Redis Standalone

+ `rediss` Redis Standalone SSL

+ `redis-socket` Redis Standalone Unix Domain Socket

+ `redis-sentinel` Redis Sentinel

**Timeout units**

+ `d` Days
+ `h` Hours
+ `m` Minutes
+ `s` Seconds
+ `ms` Milliseconds
+ `us` Microseconds
+ `ns` Nanoseconds

Hint: The database parameter within the query part has higher precedence than the database in the path.

RedisURI supports Redis Standalone, Redis Sentinel and Redis Cluster with plain, SSL, TLS and unix domain socket connections.