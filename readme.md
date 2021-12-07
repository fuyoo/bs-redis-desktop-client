# BS Redis Desktop Client
English|[中文](./readme_cn.md)

> BS Redis Desktop Client rust + tauri
> The BS redis desktop client is powered by Rust and Tauri, with very small weight, beautiful interface and faster running speed!

## why have this tool?

* more tools based on electron，too heavy and large。
* rdm so ugly for me。
* rust and tauri fast and minimal samll。
* html make me crate a pretty interface。

## Software download and communication

* Software download：[Home Page](http://bs.echosocket.com)
* Communication： QQ GROUP：877290961

## Devlopment

### Directory tree

+ `/src-tauri` 是 rust + [tauri](https://tauri.studio/) 主文件目录
+ `/ui` 是基于 [vue3](https://v3.vuejs.org/) + [element-plus](https://element-plus.org/zh-CN/) 的前端项目

### Init environment

- install `nodejs` and `rust`
- install `tauri-cli` tools
  `cargo install tauri-cli --version ^1.0.0-beta`
- cd `ui` and execute the following command
  `yarn install` or `npm install` install the dependence

### Debugger

- at `ui` directory execute `yarn dev` or `npm run dev` run front end
- at other terminal exceute `cargo tauri dev` run tauri 

### Other

- packake：at `src-tauri` dir execute `cargo tauri build`
- app icon：at `ui` dir  execute `yarn tauri icon "image absoute path"` or `npm run tauri icon "image absoute path"`


## 软件预览

**software home**
![1](https://raw.githubusercontent.com/fuyoo/bs-redis-desktop-client/master/imgs/1.png)

**redis base info**
![2](https://raw.githubusercontent.com/fuyoo/bs-redis-desktop-client/master/imgs/2.png)

**key details**
![3](https://raw.githubusercontent.com/fuyoo/bs-redis-desktop-client/master/imgs/3.png)

**redis config details**

![4](https://raw.githubusercontent.com/fuyoo/bs-redis-desktop-client/master/imgs/4.png)

**Pub/Sub Debugger**
![5](https://raw.githubusercontent.com/fuyoo/bs-redis-desktop-client/master/imgs/5.png)

## if you are interested for this project please make a PR! thanks a lot!

## LICENSE
[MIT](./LICENSE)
