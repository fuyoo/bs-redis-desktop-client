import databaseMenu from './src/index'
export const DatabaseMenu = databaseMenu

const install = app => app.component(DatabaseMenu.name, DatabaseMenu)
export default {
  install,
  DatabaseMenu
}
