import {getSize} from "./util";

export const view = Window.this


export function custom_dialog(config) {
    return new Promise((resolve, reject) => {
        setTimeout(() => {
            resolve(view.modal(config))
        })
    })
}

export function add_connection_dialog() {
    let [x,y,w,h] = getSize();
    let left = x + w / 2 - 200 / 2
    let top = y + h / 2 - 160 / 2
    return custom_dialog({
        type: Window.DIALOG_WINDOW,
        parent: view,
        alignment: 5,
        width: 300,
        height:280,
        x: left,
        y: top,
        url: "this://app/dialog/connection.html"
    })
}


