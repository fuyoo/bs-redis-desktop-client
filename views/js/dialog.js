import {getSize} from "./util";
import {devicePixels} from "@sciter"
export const view = Window.this


export function custom_dialog(config) {
    return new Promise((resolve, reject) => {
        setTimeout(() => {
            resolve(view.modal(config))
        })
    })
}

export function connection_dialog(parameters) {
    let [x,y,w,h] = getSize();
    let left = x + w / 2 - 300 / 2
    let top = y + h / 2 - 280 / 2
    left = devicePixels(left)
    top = devicePixels(top)
    return custom_dialog({
        type: Window.DIALOG_WINDOW,
        parent: view,
        alignment: 5,
        width: devicePixels(300),
        height:devicePixels(280),
        x: left,
        y: top,
        url: "this://app/dialog/connection.html",
        parameters
    })
}


