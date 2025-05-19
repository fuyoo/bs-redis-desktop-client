import type { TreeOption } from 'naive-ui'
import type { VNode } from 'vue'

export type RedisKeyType = 'string' | 'hash' | 'set' | 'list' | 'zset' | 'stream'|''


// tree object
export  interface Tree extends TreeOption {
  label: string
  value?: string
  icon: string
  type: 'key' | 'folder'
  id: string
  children?: Tree[]
  prefix?: () => VNode
}
