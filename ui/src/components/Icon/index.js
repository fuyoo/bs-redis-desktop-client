import icon from './src/Icon'

export const Icon = icon
export default {
  install(vue) {
    vue.component(icon.name, icon)
  }
}
