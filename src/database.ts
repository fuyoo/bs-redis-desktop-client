import Dexie, {Table} from 'dexie'

export interface ConnectionImpl {
    id?: number;
    name: string;
    uri: string | string[];
    cluster: boolean;
}
export interface SettingsImpl {
    id?: number;
    name: string;
    uri: string | string[];
    cluster: boolean;
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