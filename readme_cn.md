# BS Redis Desktop Client

中文|[English](./readme.md)

> BS Redis Desktop Client rust + tauri

> BS redis桌面客户端由Rust和Tauri提供动力，体积非常小，界面美观，运行速度非常快！

## 分支sciter基于sciter

## 为什么有这个工具?

* 太多的工具基于electron，太大太重。
* rdm 对于我来说太丑了。
* 基于rust和tauri的工具又快又轻。
* 基于html的tauri让我可以轻松创建一个漂亮的GUI界面。

## 软件下载及交流

* 软件下载：[软件官网](http://bs.echosocket.com)
* 交流： QQ群：877290961

## 开始开发

### 目录结构

+ `/src-tauri` 是 rust + [tauri](https://tauri.studio/) 主文件目录
+ `/ui` 是基于 [vue3](https://v3.vuejs.org/) + [element-plus](https://element-plus.org/zh-CN/) 的前端项目

### 初始化环境

- 安装 nodejs 和 rust
- 安装tauri-cli工具
  `cargo install tauri-cli --version ^1.0.0-beta`
- 到ui目录执行
  `yarn install` 或者 `npm install` 安装依赖，

### 本地调试

- 在ui目录中执行 `yarn dev` 或者 `npm run dev` 启动ui界面
- 在另起一个窗口，在当前目录下执行 `cargo tauri dev`

### 其他

- 项目打包：在当前目录下执行 `cargo tauri build`
- 更改图标：在ui目录执行 `yarn tauri icon 图片绝对路径` 或者 `npm run tauri icon 图片绝对路径`


## 软件预览

**软件主界面**
![1](https://raw.githubusercontent.com/fuyoo/bs-redis-desktop-client/master/imgs/1.png)

**redis基本信息**
![2](https://raw.githubusercontent.com/fuyoo/bs-redis-desktop-client/master/imgs/2.png)

**key查看界面**
![3](https://raw.githubusercontent.com/fuyoo/bs-redis-desktop-client/master/imgs/3.png)

**redis配置信息详情**

![4](https://raw.githubusercontent.com/fuyoo/bs-redis-desktop-client/master/imgs/4.png)

**Pub/Sub调试**
![5](https://raw.githubusercontent.com/fuyoo/bs-redis-desktop-client/master/imgs/5.png)

## 欢迎各位有兴趣的朋友提交PR

## LICENSE
[MIT](./LICENSE)
