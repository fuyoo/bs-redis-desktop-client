module.exports = {
  method: 'get',
    auth: false,
  path: '/c/:c',// request http://url/home/id?q=1 by get method
  handler: async ({ params, query, body, headers }) => {
  return process.response.ok(params)
}
}
