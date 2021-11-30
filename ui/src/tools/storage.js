/**
 * author fuyoo
 * 封装一下localStorage || sessionStorage
 */
/**
 *
 * @param cache
 * @returns {string|{set(*=, *=): Promise<*|undefined>, get(*=): Promise<any|string|undefined>, clear(): Promise<void>, del(*=): Promise<void>}|any}
 */
const instance = cache => {
  const storage = cache === true ? sessionStorage : localStorage
  return {
    async getItem(key) {
      const val = storage.getItem(key)
      try {
        return JSON.parse(val, (key, value) => {
          if (value.indexOf('function') > -1) {
            return eval('(function(){return ' + value + ' })()')
          }
          return value
        })
      } catch (e) {
        return val
      }
    },
    async setItem(key, val) {
      let data = ''
      try {
        data = JSON.stringify(val, (key, value) => {
          if (typeof value === 'function') {
            return value.toString()
          }
          return value
        })
      } catch (e) {
        data = val
      }

      storage.setItem(key, data)
    },
    async removeItem(key) {
      storage.removeItem(key)
    },
    async clear() {
      storage.clear()
    }
  }
}
/**
 * session 代表 sessionStorage 默认情况下使用localStorage
 */
export default Object.assign(instance(false), { session: instance(true) })
