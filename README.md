# Pot-App Cohere AI 翻译插件

## 特征
- 支持 Command R+, Command R, Command, Command Nightly, Command Light, Command Light Nightly .
- 暂不支持流响应。
- 支持翻译、润色、总结、自定义提示。

## 支持平台

- [x] Windows
  - [x] x64 [下载](https://gh.pylogmon.com/https://github.com/imShire/pot-app-translate-plugin-cohere/releases/latest/download/x86_64-pc-windows-msvc.zip)
  - [x] x86 [下载](https://gh.pylogmon.com/https://github.com/imShire/pot-app-translate-plugin-cohere/releases/latest/download/i686-pc-windows-msvc.zip)
  - [x] aarch64 [下载](https://gh.pylogmon.com/https://github.com/imShire/pot-app-translate-plugin-cohere/releases/latest/download/aarch64-pc-windows-msvc.zip)
- [x] Linux
  - [x] x64 [下载](https://gh.pylogmon.com/https://github.com/imShire/pot-app-translate-plugin-cohere/releases/latest/download/x86_64-unknown-linux-gnu.zip)
  - [x] x86 [下载](https://gh.pylogmon.com/https://github.com/imShire/pot-app-translate-plugin-cohere/releases/latest/download/i686-unknown-linux-gnu.zip)
  - [x] aarch64 [下载](https://gh.pylogmon.com/https://github.com/imShire/pot-app-translate-plugin-cohere/releases/latest/download/aarch64-unknown-linux-gnu.zip)
  - [x] armv7 [下载](https://gh.pylogmon.com/https://github.com/imShire/pot-app-translate-plugin-cohere/releases/latest/download/armv7-unknown-linux-gnueabihf.zip)
- [x] MacOS
  - [x] x64 [下载](https://gh.pylogmon.com/https://github.com/imShire/pot-app-translate-plugin-cohere/releases/latest/download/x86_64-apple-darwin.zip)
  - [x] aarch64 [下载](https://gh.pylogmon.com/https://github.com/imShire/pot-app-translate-plugin-cohere/releases/latest/download/aarch64-apple-darwin.zip)

## 使用方法

1. 下载对应平台的插件，解压得到 `.potext` 文件
2. 打开Pot-偏好设置-服务设置-翻译-添加外部插件-安装外部插件
3. 选择刚刚解压得到的 `.potext` 文件，安装成功
4. 将插件添加到服务列表即可使用

## 设置代理域名
```sh
# cohere.json
{
  "version": 2,
  "routes": [
    {
      "src": "/(.*)",
      "dest": "https://api.cohere.ai/$1",
      "headers": {
        "Cache-Control": "no-cache",
        "Host": "api.cohere.ai",
        "origin": "https://api.cohere.ai",
        "referer": "https://api.cohere.ai/",
        "user-agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36"
      }
    }
  ]
}
```
执行 `vercel -A cohere.json --prod` 并绑定域名部署代理


## 作者
pot-app-translate-plugin-cohere © [imShire](https://github.com/imShire), Released under the [GPL 3.0](https://github.com/imShire/pot-app-translate-plugin-cohere/blob/main/LICENSE) License.

