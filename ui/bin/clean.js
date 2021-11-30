/**
 * @author fuyoo
 * @description 清除打包的文件夹
 */
const fs = require('fs')
const { output } = require('../config/webpack.config')
fs.rmdirSync(output.path, { recursive: true })
console.log('clean success')
