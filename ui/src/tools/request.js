const axios = require('axios')
const api_url = process.env[ 'api_url' ] || ''
const instance = axios.create({
  baseURL: api_url
})
instance.interceptors.request.use(config => {
  return config
}, error => {
  return error
})
instance.interceptors.response.use(response => {
  return response
}, error => {
  return error
})
export default instance
