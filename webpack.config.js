const path = require("path");

const webpack = require("webpack");
const { CheckerPlugin } = require('awesome-typescript-loader');
const HtmlWebpackPlugin = require("html-webpack-plugin");
const ExtractTextPlugin = require("extract-text-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry: {
    main: './src/index.ts'
  },
  output: {
    path: path.resolve(__dirname, "public"),
    filename: "[name].[chunkhash].chunk.js",
    chunkFilename: "[chunkhash].[id].js"
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js', '.wasm']
  },
  module: {
    rules: [
      {
        test: /\.css$/,
        use: ['style-loader', 'css-loader'],
      },
      {
        test: /\.(tsx|ts)?$/,
        use: 'awesome-typescript-loader',
        exclude: /node_modules/
      },
    ],
  },
  plugins: [
    new CheckerPlugin(),

    new HtmlWebpackPlugin(),

    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "runtime"),
    }),
  ]
}
