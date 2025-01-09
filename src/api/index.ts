import { type ConnectionHost } from '@/db'
import { invoke } from '@tauri-apps/api/core'
import { Notify } from 'quasar'

export const request = async <T>({
  connectionInfo,
  path,
  data,
  notify,
}: {
  connectionInfo: ConnectionHost
  path: string
  data?: string
  notify?: boolean
}): Promise<BackendResponse<T>> => {
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
      position: 'top',
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
