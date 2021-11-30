/**
 * @author fuyoo
 * @description 模拟数据处理方法
 */
const fs = require('fs')
const path = require('path')
// 加载配置文件
const cfg = require('./load_config')()
// 解析body中间件
const bodyParser = require('body-parser')
// 获取js文件路径
const mocks = path.resolve(__dirname, '../mock')
// 递归读取文件
const readDir = (pathname, list) => {
  try {
    const stat = fs.statSync(pathname)
    if (stat.isDirectory()) {
      let dirs = fs.readdirSync(pathname)
      dirs.forEach(item => readDir(path.join(pathname, item), list))
    }
    if (stat.isFile() && /.+\.js$/.test(pathname)) {
      list.push(pathname)
    }
  } catch (e) {
    return
  }
}
//自动生成pathname
const parsePath = p => {
  return p.replace(mocks, '').replace('.js', '').replace(/\\/g, '/')
}
const varType = val => Object.prototype.toString.call(val).slice(8, -1)
/**
 * @param app {ExpressInstance}
 */
module.exports = app => {
  // 加载中间件
  app.use(bodyParser.urlencoded({ extended: true }))
  // 解析为json
  app.use(bodyParser.json())
  // 抓一波异常
  try {
    // 获取mockdata下的路径
    const files = []
    // 读取文件
    readDir(mocks, files)
    // 开始生成路由
    files.forEach(item => {
      // 提取控制器
      // 删除缓存
      delete require.cache[ require.resolve(item) ]
      let controllers = require(item)
      // control生成器
      const generator = ({ method, path, handler, auth }) => {
        // 配置默认数据
        method = method || 'get'
        path = path || item
        handler = handler || (params => ({}))
        auth = auth || false
        // 加载路由
        app[ method.toLowerCase() ](parsePath(path || item), async (req, res) => {
          try {
            // 判断是否授权
            if (auth) {
              const token_key = req.headers[ cfg.auth_key ]
              if (!token_key || token_key != cfg.auth_val) {
                res.json(JSON.parse(cfg.auth_response || {}))
                return
              }
            }
            const { type, data, headers } = await handler({
              query: req.query,
              params: req.params,
              body: req.body,
              headers: req.headers
            })
            if (headers) {
              Object.keys(headers).forEach(key => {
                res.setHeader(key, headers[ key ])
              })
            }
            if (!type || type === 'json') {
              res.json(data)
            }
            if (type === 'blob') {
              res.end(data)
            }
          } catch (e) {
            res.json(e)
          }
        })
      }
      // 开始生成
      switch (varType(controllers)) {
        case 'Array':
          controllers.forEach(control => {
            // 组合相关数据
            let { method, path, handler } = require(item)
            generator(control)
          })
        case 'Object':
          generator(controllers)
      }
    })
  } catch (e) {
    console.log(e)
  }
}
