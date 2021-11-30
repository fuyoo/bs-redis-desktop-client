/**
 * @author fuyoo
 * @description 解析配置文件
 */
const path = require('path')
const fs = require('fs')
module.exports = () => {
  // 根据环境来判断使用那个配置文件
  const filename = process.env.NODE_ENV === 'development' ? 'development' : 'production'
  // 组合配置文件路径
  const cfg = path.resolve(__dirname, `../config/${ filename }.cfg`)
  // 抓一下异常
  try {
    // 定义个初始的配置文件
    const data = {}
    // 获取内容
    let content = fs.readFileSync(cfg).toString()
    // 提取内容
    let arr = content.split('\n')
    // 过滤
    arr = arr.filter(e => /^(\s)*[^#]+=(.+)$/ig.test(e))
    arr.forEach(e => {
      // 解析内容
      let item = e.trim()
      if (item.indexOf('#') > -1) {
        item = item.slice(0, item.indexOf('#'))
      }
      let [key, val] = item.split('=')
      key = key.trim()
      data[ key ] = val.trim()
    })
    // 返回结果
    return data
  } catch (e) {
    return {}
  }
}
