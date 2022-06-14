const view = Window.this

export function getSize() {
    return view.box("xywh", "border", "desktop")
}

export function request(path, data, parse) {
    return new Promise((resolve, reject) => {
        setTimeout(() => {
            try {
                if (parse) {
                    data = JSON.parse(data)
                }
                view.xcall("fetch", path, !!data ? data : "", res => {
                    try {
                        resolve(JSON.parse(res))
                    } catch (e) {
                        reject(e)
                    }
                })
            } catch (e) {
                reject(e)
            }
        })
    })
}