import { type ConnectionHost } from '@/db'
import { invoke } from '@tauri-apps/api/core'
import { Notify } from 'quasar'

export const request = async <T>({
  connectionInfo,
  path,
  data,
}: {
  connectionInfo: ConnectionHost
  path: string
  data?: string
}): Promise<BackendResponse<T>> => {
  // fetch response
  const resp = await invoke<string>('request', {
    path,
    connectionInfo,
    data,
  })

  const body = JSON.parse(resp || '{"code":-1,"msg":"Response is empty"}') as BackendResponse<T>
  // if code is not 0, show error message
  if (body.code !== 0) {
    Notify.create({
      position: 'top',
      message: body.msg,
      color: 'negative',
      icon: 'report_problem',
      timeout: 3000,
    })
  }
  return body
}
