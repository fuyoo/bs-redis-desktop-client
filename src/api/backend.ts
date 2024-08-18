import db from "@/database"
import {invoke} from "@tauri-apps/api/core"
import {useTabStore} from "@/store/tabs.ts";

export interface ResponseBody<T> {
    code: number,
    data: T,
    msg: string
}

export const Req = async <T>(path: string, data: any) => {
    const tabs = useTabStore()
    const id = Number(tabs.activeTab?.id || '0')
    const info = await db.connection.get(id);
    if (typeof data == "object") {
        data = JSON.stringify(data)
    }
    const res = await invoke<string>("request", {
        path,
        data,
        connectionInfo: info
    })
    return JSON.parse(res) as ResponseBody<T>
}


export function status<T>(data: any) {
    return Req<T>("/status", data)
}

export enum InfoSection {
    Server = "Server",
    Clients = "Clients",
    Memory = "Memory",
    Persistence = "Persistence",
    Stats = "Stats",
    Replication = "Replication",
    CPU = "CPU",
    Modules = "Modules",
    ErrorStats = "Errorstats",
    Cluster = "Cluster",
    Keyspace = "Keyspace",
    All = ""
}

export function info<T>(data: InfoSection) {
    return Req<T>("/info", data)
}