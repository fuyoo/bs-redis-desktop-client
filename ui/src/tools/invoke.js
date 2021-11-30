import {invoke} from '@tauri-apps/api/tauri'
import {alert} from ':/tools/alert';

export const request = (eventName, params) => {
    return new Promise((resolve, reject) => {
        invoke(eventName, params)
            .then(res => {
                if (!res) {
                    resolve(res)
                    return
                }
                if (res.code !== 200) {
                    throw res.msg
                }
                resolve(res)
            })
            .catch(e => {
                alert.error(e.toString())
                reject(e)
            })
    })
}

