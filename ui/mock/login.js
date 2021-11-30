const cfg = require('../bin/load_config')()

module.exports = {
  method: 'post',
  path: '/login',
  handler: async ({ params, query, body, headers }) => {
    const { username, password } = body
    if (username === 'fuyoo' && password === '123456') {
      return process.response.ok(cfg.auth_val)
    } else {
      return process.response.fail(500, '用户名或密码错误')
    }
  }
}
