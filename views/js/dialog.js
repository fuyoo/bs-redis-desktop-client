export const view = Window.this


export function custom_dialog(config) {
    return new Promise((resolve, reject) => {
        setTimeout(() => {
            resolve(view.modal(config))
        })
    })
}

export function add_connection_dialog() {
    return custom_dialog({
        type: Window.DIALOG_WINDOW,
        parent: view,
        alignment: 5,
        url: "this://app/dialog/connection.html"
    })
}


