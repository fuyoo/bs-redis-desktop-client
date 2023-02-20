# BS Redis Desktop Client n
English | [中文](./readme_cn.md)

> BS Redis Desktop Client tauri

> The BS redis desktop client is powered by Rust and Tauri, with very small weight, beautiful interface and faster running speed!

## this project is rebuilding, if you need the old resource please check tag#1.1.1

## branch sciter is based on sciter framework,more info see sciter branch. 

## Software download and communication

* Software download：[Home Page](http://bs.xsa.link)
* Communication： QQ GROUP：877290961

## Devlopment

### Directory tree

+ `/src-tauri` is  [tauri](https://tauri.studio/) tauri project
+ `bs-frontend` is the project frontend and it's a git submodule, just pack a gui resource

### Init environment

- install `nodejs` and `rust`
- install `tauri-cli` tools, execute `cargo install tauri-cli`

### Debugger
1. clone the `bs-frontend` repo [bs-frontend](https://github.com/fuyoo/bs-frontend.git) and read the readme.md to run it.  
2. at this project directory execute `cargo tauri dev` 

### build
1. update git submodule execute `git submodule init && git submodule update --remote`.
2. cd `bs-frontemd` execute `pnpm i` or `npm i` install all dependencies.
3. execute `pnpm build` or `npm run build`,package the frontend resource.
4. at the project root directory execute `cargo tauri build`.
5. end.

### Other

- change app icon：at this project directory execute `cargo tauri icon "image absoute path"`


## Tag#1.1.1 Screenshots, new version is developing

![1](https://raw.githubusercontent.com/fuyoo/bs-redis-desktop-client/master/imgs/en_0.png)

![2](https://raw.githubusercontent.com/fuyoo/bs-redis-desktop-client/master/imgs/en_1.png)

![3](https://raw.githubusercontent.com/fuyoo/bs-redis-desktop-client/master/imgs/en_2.png)

![4](https://raw.githubusercontent.com/fuyoo/bs-redis-desktop-client/master/imgs/en_3.png)

## if you are interested for this project please make a PR! thanks a lot!

## LICENSE
[MIT](./LICENSE)
