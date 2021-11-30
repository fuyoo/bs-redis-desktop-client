import {listen} from '@tauri-apps/api/event';
import store from ':/store';
import {getCurrent, WebviewWindow} from '@tauri-apps/api/window';
import {request} from ':/tools/invoke';
import set from '::/set';

!(async () => await listen('pubsub', async evt => {
    const {address, channel, port} = evt.payload
    let add0 = t => t > 10 ? '' + t : '0' + t;
    let date = new Date();
    let y = date.getFullYear()
    let m = add0(date.getMonth() + 1)
    let d = add0(date.getDay())
    let h = add0(date.getHours())
    let i = add0(date.getMinutes())
    let s = add0(date.getSeconds())
    await store.dispatch('insertPubSubData', {
        channel: `${address}:${port}/${channel}`,
        data: {time: `${y}-${m}-${d} ${h}:${i}:${s}`, ...evt.payload}
    })
}))()
!(async () => await listen('set', (event) => {
    new WebviewWindow('app-set', {
        url: '/#/set',
        title: '软件设置',
        width: 360,
        height: 280,
        center: true,
        visible: false,
        alwaysOnTop: true,
        skipTaskbar: true,
        resizable: false,
    })
}))()
!(async () => await listen('single', (event) => {
    console.log("get signal")
    let current = getCurrent()
    current.setAlwaysOnTop(true)
    current.show()
    setTimeout(()=>{
        current.setAlwaysOnTop(false)
    },50)
}))()

!(async  ()=> await request("listen_single"))()