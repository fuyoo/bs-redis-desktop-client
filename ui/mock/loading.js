const fs = require("fs/promises");
const path = require("path")
module.exports = {
  method: 'get',
  path: '/loading.html',// request http://url/home/id?q=1 by post method
  handler: async ({ params, query, body, headers }) => {
    let file = await fs.readFile(path.join(__dirname,"../public/loading.html"))
    return {
      type: 'blob',
      data: file,
      headers: {
        'Content-Type': 'text/html;charset=utf-8'
      }
    }
  }
}
