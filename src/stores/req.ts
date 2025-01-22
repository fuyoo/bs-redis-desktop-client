import { defineStore } from 'pinia'

import { db, type ConnectionHost } from '@/db'
import { invoke } from '@tauri-apps/api/core'
import { Notify } from 'quasar'
import { useRoute } from 'vue-router'
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
    Notify.create({
      position: 'bottom',
      message: body.msg,
      color: 'negative',
      timeout: 0,
      attrs: {
        style: 'line-height: 1;',
      },
      actions: [{ icon: 'close', dense: true, rounded: true, handler: () => {}, color: 'yellow' }],
    })
  }
  return body
}

export const useReqStore = defineStore('req', () => {
  // at here, we should get host from route params.
  const route = useRoute()
  const reqWithHost = async <R>(option: {
    path: string
    data?: any
    db?: string
    notify?: boolean
  }): Promise<BackendResponse<R>> => {
    const host = await db.hosts.get({ id: parseInt(route.params.id as string) })
    if (option.db && host) {
      host.node[0].db = option.db
    }
    return request({
      connectionInfo: host!,
      path: option.path,
      data: option.data,
      notify: option.notify,
    })
  }
  const reqNoHost = request
  return { reqWithHost, reqNoHost }
})
