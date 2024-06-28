import { useRoute } from "vue-router"
import db from "@/database"
import { invoke } from "@tauri-apps/api/core"
const getConnectionInfo = async () => {
    const route = useRoute()
    const id = Number(route.query.id)
    return await db.connection.get(id)
} 

export interface ResponseBody<T> {
    code: number,
    data:T,
    msg: string
}

export const RReq  = async <T>(action:string, data: any) => {
        const info =  await getConnectionInfo()
        return invoke<ResponseBody<T>>("request",{
            rid:Math.random().toString(36),
            action,
            data:JSON.stringify(data),
            connectionInfo:JSON.stringify(info)
        })
}


export function status<T>(data:any) {
    return RReq("/status",data)
}
