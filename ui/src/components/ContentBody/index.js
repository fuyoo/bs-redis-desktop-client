import contentBody from './src/index'

export const ContentBody = contentBody
const install = app => app.component(ContentBody.name, ContentBody)
export default {
  install,
  ContentBody
}
