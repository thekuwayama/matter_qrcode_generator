const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')

module.exports = {
    mode: 'none',
    entry: './www/js/app.js',
    output: {
        path: path.resolve(__dirname, 'public'),
        filename: '[hash].js'
    },
    plugins: [
        new HtmlWebpackPlugin({
            inject: true,
            template: "./www/index.html"
        })
    ],
    module: {
        rules: [
            {
                test: /\.css$/,
                use: ['style-loader', 'css-loader']
            }
        ]
    }
}
