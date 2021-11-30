/**
 * @author fuyoo
 * @description 监听文件夹，刷新路由
 */

const fs = require('fs')
const path = require('path')
fs.watch(path.resolve(__dirname, '../mock'), { recursive: true }, (type, file) => {
  process.send({
    type,
    file
  })
})
