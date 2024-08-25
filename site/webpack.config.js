const CopyPlugin = require("copy-webpack-plugin");
const path = require("path");
const { LoaderOptionsPlugin } = require("webpack");

module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  mode: "development",
  experiments: {
    asyncWebAssembly: true,
  },
  plugins: [
    new CopyPlugin({
      patterns: [{ from: "index.html" }, { from: "assets/my_weights.json", to: "assets/my_weights.json" }],
    })
  ],
  devServer: {
    headers: { "Access-Control-Allow-Origin": "*" }
 }
};
