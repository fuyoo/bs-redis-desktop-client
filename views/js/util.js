const view = Window.this

export function getSize() {
    return view.box("xywh", "border", "desktop")
}

export function request(path, data, stringify) {
    return new Promise((resolve, reject) => {
        setTimeout(() => {
            try {
                if (stringify === true) {
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