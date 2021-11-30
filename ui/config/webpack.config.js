/**
 * @author fuyoo
 * @description webpack配置文件
 */
const fs = require('fs')
const path = require('path')
const chalk = require('chalk')
const { fork } = require('child_process')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const { CleanWebpackPlugin } = require('clean-webpack-plugin')
const { VueLoaderPlugin } = require('vue-loader')
const { DefinePlugin } = require('webpack')
const mode = process.env.NODE_ENV
const output = path.resolve(__dirname, '../dist')
const ip = require('@fuyoo/real_ip')
const cfg = require('../bin/load_config')()
const mocks = require('../bin/load_mockdata')
process.response = require('../bin/format_res')
const app_name = JSON.parse(fs.readFileSync(path.resolve(__dirname, '../package.json')).toString()).name
const prod = require('./webpack.prod.js')
const ExtractCssLoader = () => {
  if (mode === 'production') {
    return prod().loaders
  }
  return ['style-loader', { loader: 'css-loader', options: { import: false } }]
}
module.exports = {
  mode,
  devtool: mode === 'production' ? false : 'eval-source-map',
  entry: {
    // 多入口配置展示
    main1: path.resolve(__dirname, '../src/main'),
    // 多入口配置展示
    main2: path.resolve(__dirname, '../src/main')
  },
  output: {
    path: output,
    publicPath: '/',
    filename: 'static/js/[name].[hash:8].js',
  },
  resolve: {
    alias: {
      ':': path.resolve(__dirname, '../src'),
      '::': path.resolve(__dirname, '../src/views')
    },
    extensions: ['.js', '.vue', '.sass', '.json']
  },
  devServer: {
    contentBase: output,
    compress: true,
    host: cfg.dev_host || '0.0.0.0',
    port: cfg.dev_port || 9000,
    inline: true,
    hot: true,
    overlay: true,
    before: function (app) {
      // 生成mock接口
      mocks(app)
      const child = fork('./bin/listen.js')
      child.on('message', data => {
        process.stdout.write(chalk.blue(`Mock directory has been changed, regenerating routes.\n`))
        // 重置路由
        app._router.stack = app._router.stack.filter(e => e.route == undefined || e.route.path.indexOf('webpack') > -1)
        // 重新生成
        mocks(app)
        process.stdout.write(chalk.green('New routes has been regenerated.\n'))
      })
    },
    after(app, server, compiler) {
      //编译完事件
      compiler.hooks.done.tap('Done', ({ compilation }) => {
        if (compilation.errors.length > 0) {
          return
        }
        setTimeout(async () => {
          // 获取本地ip
          const res = await ip()
          // 获取输出
          const stdout = process.stdout
          // 获取配置
          const options = compiler.options.devServer
          // 写信息
          stdout.write(chalk.green('\n Resource compiled successfully.\n Development server has been started.'))
          stdout.write(chalk.blue('\n > Network listen at: http://' + res + ':' + options.port))
          stdout.write(chalk.blue('\n > Local   listen at: http://127.0.0.1:' + options.port))
          // 填充到最底层
          for (let i = 0; i < stdout.rows - 5; i++) {
            stdout.write('\n')
          }
          // 重置光标
          process.stdout.moveCursor(1, -(stdout.rows - 6))
        }, 500)
      })
    }
  },
  stats: 'minimal',
  module: {
    rules: [
      {
        test: /\.(ttf|eot|svg|woff|woff2)$/,
        type: 'asset/resource',
        generator: {
          filename: 'static/fonts/[hash][ext][query]'
        }
      },
      {
        test: /\.(png|jpe?g|gif)$/i,
        type: 'asset/resource',
        generator: {
          filename: 'static/images/[hash][ext][query]'
        }
      },
      {
        test: /\.css$/i,
        use: [
          ...ExtractCssLoader()
        ]
      },
      {
        test: /\.s[ac]ss$/i,
        use: [
          ...ExtractCssLoader(),
          {
            loader: 'sass-loader',
            options: {
              // Prefer `dart-sass`
              implementation: require('sass'),
              sourceMap: true
            }
          },
          {
            loader: 'sass-resources-loader',
            options: {
              sourceMap: true,
              // Or array of paths
              resources: [
                path.resolve(__dirname, '../src/style/common.scss')
              ]
            }
          }
        ]
      },
      {
        test: /\.m?js$/,
        exclude: /(node_modules|bower_components)/,
        use: {
          loader: 'babel-loader',
          options: {
            presets: ['@babel/preset-env'],
            plugins: ['@vue/babel-plugin-jsx', '@babel/plugin-transform-runtime']
          }
        }
      },
      {
        test: /\.vue$/,
        use: [{
          loader: 'vue-loader'
        }]
      }
    ]
  },
  plugins: [
    new CleanWebpackPlugin(),
    new VueLoaderPlugin(),
    new DefinePlugin({
      'process.env': JSON.stringify(Object.assign({ NODE_ENV: process.env.NODE_ENV }, cfg)),
      '__VUE_OPTIONS_API__': JSON.stringify(true),
      '__VUE_PROD_DEVTOOLS__': mode === 'production' ? JSON.stringify(false) : JSON.stringify(true)
    }),
    // 多入口配置展示
    new HtmlWebpackPlugin({  // Also generate a test.html
      title: app_name,
      filename: 'index.html',
      template: path.resolve(__dirname, '../public/index.html'),
      chunks: ['main1']
    }),
    // 多入口配置展示
    new HtmlWebpackPlugin({  // Also generate a test.html
      title: app_name,
      filename: 'loading.html',
      template: path.resolve(__dirname, '../public/loading.html')
    }),
    ...prod().plugins
  ],
  optimization: prod().optimization
}
