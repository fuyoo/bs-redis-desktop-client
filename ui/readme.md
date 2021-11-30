# vue3-quickstart-template

## features

+ vue3
+ no vue-cli
+ element-ui vue3 version
+ easy to mock api

## easy mock an api just need following two steps

1. create an api.js file at the mock directory.
2. write some node.js script in the api.js.
   ```javascript
   module.exports = {
        method: 'get',
        path: '/home/:id',// request http://url/home/id?q=1 by get method
        handler: async ({ params, query, body, headers }) => {
            return process.response.ok({
                params,
                query,
                body,
                headers
            })
        }
   }
    ```

> when the process started.
> process will watch the mock directory auto generate api route if the directory has been changed
> so if your write complete, api has been auto generated!
> more example please look `mock` directory

## response format function

> there have three functions mounted at the global process object.

* first is `ok` `process.response.ok(data,headers)` .
* second is `fail` `process.response.fail(code,msg,headers)` .
* third is `format` `process.response.format(code,data,msg,headers)`.
* all the three function's details please look the `/bin/format_res.js` file.
* of course custom it by your self.

## config webstorm support webpack resolve.alias

* open webstorm preference
* search webpack
* choose Manually, click right button and select `/config/webpack.config.js` file

## webpack alias fields

* **:** is `~/src` alias
* **::** is `~/src/views` alias

## configure

* development configure file at `config/development.cfg`
* production configure file at `config/configure.cfg`

## LICENCES

[MIT LICENCES](LICENSE)
