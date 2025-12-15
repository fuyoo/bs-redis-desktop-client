import { defineStore } from 'pinia'

import { db, type ConnectionHost } from '@/db'
import { invoke } from '@tauri-apps/api/core'
import { useRoute } from 'vue-router'
import { ref } from 'vue'
import { message } from '@/tools'
export interface CommonRequestParams {
  connectionInfo: ConnectionHost
  path: string
  data?: string
  notify?: boolean
}
const request = async <T>({
  connectionInfo,
  path,
  data,
  notify,
}: CommonRequestParams): Promise<BackendResponse<T>> => {
  // fetch response
  const resp = await invoke<string>('request', {
    path,
    connectionInfo,
    data,
  })

  const body = JSON.parse(resp || '{"code":-1,"msg":"Response is empty"}') as BackendResponse<T>
  // if code is not 0, show error message
  if (body.code !== 0 && notify !== false) {
    message.warning(body.msg)
  }
  return body
}
const _T = (v: any) => Object.prototype.toString.call(v).slice(8, -1)
export const useReqStore = defineStore('req', () => {
  const reqLoading = ref(false)
  // at here, we should get host from route params.
  const route = useRoute()
  // define a  timer to throttle the loading status
  let timer: any
  const reqWithHost = async <R>(option: {
    path: string
    data?: any
    db?: string
    notify?: boolean
  }): Promise<BackendResponse<R>> => {
    clearTimeout(timer)
    reqLoading.value = true
    const host = await db.hosts.get({ id: parseInt(route.params.id as string) })
    if (route.query.db && host) {
      host.node[0].db = route.query.db as string
    }
    if (option.db && host) {
      host.node[0].db = option.db
    }
    try {
      const resp = await request<R>({
        connectionInfo: host!,
        path: option.path,
        data: _T(option.data) != 'String' ? JSON.stringify(option.data) : option.data,
        notify: option.notify,
      })
      // throttle timer
      timer = setTimeout(() => (reqLoading.value = false), 100)
      return resp
    } catch (e) {
      // throttle timer
      timer = setTimeout(() => (reqLoading.value = false), 100)
      throw e
    }
  }
  const reqNoHost = request
  return { reqWithHost, reqNoHost, reqLoading }
})
