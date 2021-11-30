const fs = require('fs')
const path = require('path')
module.exports = {
  method: 'get',
  path: '/home/main',
  handler: async () => {
    return new Promise((resolve, reject) => {
      fs.readFile(path.resolve(__dirname, '../../public/index.html'), (err, data) => {
        if (!err) {
          return resolve({
            type: 'blob',
            headers:{
              "content-type":"text/html"
            },
            data
          })
        }
        return reject(err)
      })
    })
  }
}
