/**
 * @author fuyoo
 * @description 格式化mock返回数据
 */
module.exports = {
  /**
   * 成功数据
   * @param data {*}
   * @param headers {object}
   * @returns {{headers, data: {msg: string, code: number, data}}}
   */
  ok(data, headers) {
    return {
      data: {
        code: 200,
        msg: '成功',
        data
      },
      headers
    }
  },
  /**
   * 失败数据
   * @param code {number}
   * @param msg {string}
   * @param headers {object}
   * @returns {{headers, data: {msg, code, data: null}}}
   */
  fail(code, msg, headers) {
    return {
      data: {
        code,
        msg,
        data: null
      },
      headers
    }
  }
  ,
  /**
   * 返回格式化数据
   * @param code {number}
   * @param data {*}
   * @param msg {string}
   * @param headers {object}
   * @returns {{headers, data: {msg, code, data}}}
   */
  format(code, data, msg, headers) {
    return {
      data: {
        code,
        msg,
        data
      }, headers
    }
  }
}
