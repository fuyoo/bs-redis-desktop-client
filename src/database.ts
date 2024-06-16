import Dexie, {Table} from 'dexie'


export interface ConnectionInfo{
    host: string,
    port?: string,
    db?: string,
    username?: string,
    password?: string
}
export interface ConnectionImpl {
    id?: number;
    name: string;
    node: ConnectionInfo[]
    cluster?: boolean;
}
export interface SettingsImpl {
    lang?: string,
    id: number
}
export class Database extends Dexie {
    // 'connection' is added by dexie when declaring the stores()
    // We just tell the typing system this is the case
    connection!: Table<ConnectionImpl>
    settings!: Table<SettingsImpl>
    constructor() {
        super('link.xsa.bs')
        this.version(1).stores({
            connection: '++id',
            settings: '++id'// Primary key and indexed props
        })
    }
}

export default new Database()