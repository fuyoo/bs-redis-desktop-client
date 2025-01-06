// db.ts
import Dexie, { type EntityTable } from 'dexie'

interface ConnectionInfo {
  host: string
  port?: string
  db?: string
  username?: string
  password?: string
}

// connection impl
interface ConnectionHost {
  id?: number
  name: string
  node: ConnectionInfo[]
  cluster?: boolean
}

const db = new Dexie('link.xsa.bs') as Dexie & {
  hosts: EntityTable<
    ConnectionHost,
    'id' // primary key "id" (for the typings only)
  >
}
// Schema declaration:
db.version(1).stores({
  hosts: '++id', // primary key "id" (for the runtime!)
})
export type { ConnectionHost, ConnectionInfo }
export { db }
