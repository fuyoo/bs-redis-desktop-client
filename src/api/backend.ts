
import db from "@/database"
import {invoke} from "@tauri-apps/api/core"
import {useTabStore} from "@/store/tabs.ts";

export interface ResponseBody<T> {
    code: number,
    data: T,
    msg: string
}

export const Req = async <T>(action: string, data: any) => {
    const tabs = useTabStore()
    const id = Number(tabs.activeTab?.id || '0')
    const info = await db.connection.get(id);
    return invoke<ResponseBody<T>>("request", {
        rid: Math.random().toString(36),
        action,
        data: JSON.stringify(data),
        connectionInfo: info
    })
}


export function status<T>(data: any) {
    return Req<T>("/status", data)
}
