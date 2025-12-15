import { type ConnectionHost } from '@/db'
import { invoke, type InvokeArgs } from '@tauri-apps/api/core'
import { message } from '@/tools'
interface CommonRequestParams {
  connectionInfo: ConnectionHost
  path: string
  data?: string
  notify?: boolean
}
export const request = async <T>({
  connectionInfo,
  path,
  data,
  notify,
}: CommonRequestParams): Promise<BackendResponse<T>> => {
  try {
    // fetch response
    const resp = await invoke<string>('request', {
      path,
      connectionInfo,
      data,
    })
    console.log(resp)

    const body = JSON.parse(resp || '{"code":-1,"msg":"Response is empty"}') as BackendResponse<T>
    // if code is not 0, show error message
    if (body.code !== 0 && notify !== false) {
      message.warning(body.msg)
    }
    return body
  } catch (e: any) {
    message.error(e)
    return {
      code: -1,
      data: {} as T,
      msg: e,
    }
  }
}

export const req = {
  do: async <T>(comand: string, data?: InvokeArgs) => {
    try {
      // fetch response
      const resp = await invoke<BackendResponse<T>>(comand, data)
      // if code is not 0, show error message
      if (resp.code !== 0) {
        message.warning(resp.msg)
      }
      return resp
    } catch (e: any) {
      message.error(e)
      return {
        code: -1,
        data: {} as T,
        msg: e,
      }
    }
  },
}
