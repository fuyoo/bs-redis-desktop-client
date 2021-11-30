/**
 * $Title$
 * author fuyoo
 */
const MiniCssExtractPlugin = require('mini-css-extract-plugin')
const CssMinimizerPlugin = require('css-minimizer-webpack-plugin')
const TerserPlugin = require('terser-webpack-plugin')
const isProd = process.env.NODE_ENV === 'production'
const loaders = [
  {
    loader: MiniCssExtractPlugin.loader,
    options: {
      publicPath: '/'
    }
  },
  'css-loader'
]
const plugins = [new MiniCssExtractPlugin({
  filename: 'static/css/[id].[hash:8].css'
})]
const optimization = {
  minimize: true,
  minimizer: [
    new CssMinimizerPlugin(),
    new TerserPlugin()
  ]
}
module.exports = () => {
  if (isProd) return {
    loaders,
    plugins,
    optimization
  }
  return {
    loaders: [],
    plugins: [],
    optimization: {}
  }
}
