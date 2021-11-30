import pubSub from './src/index'

export const PubSub = pubSub

const install = app => app.component(PubSub.name, PubSub)
export default {install}