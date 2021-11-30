module.exports = [{
  method: 'post',
  path: '/home/:id',// request http://url/home/id?q=1 by post method
  auth: true,
  handler: async ({ params, query, body, headers }) => {
    return process.response.format(200, {
      params,
      query,
      body,
      headers
    }, '请求成功')
  }
}, {
  method: 'get',
  auth: false,
  path: '/home/:id',// request http://url/home/id?q=1 by get method
  handler: async ({ params, query, body, headers }) => {
    return process.response.ok({
      params,
      query,
      body,
      headers
    })
  }
}]
