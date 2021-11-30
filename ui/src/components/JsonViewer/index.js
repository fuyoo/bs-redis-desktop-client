import jsonViewer from "./src/index"

export const JsonViewer = jsonViewer

const install = app => app.component(JsonViewer.name, JsonViewer)

export default {
    install,
    JsonViewer
}