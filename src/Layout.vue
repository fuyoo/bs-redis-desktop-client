<script setup lang="ts">
import {appWindow} from '@tauri-apps/api/window'
import NavigatorItem from './components/NavigatorItem.vue'
import logo from "./assets/logo.png"
import {ref} from 'vue'

const min = ref(false)
const ver = window.__SYSTEM_INFO__.pkg.version
enum BarAction {
  exit,
  mini,
  toggleMax,
}

function doWindowAction(act: BarAction) {
  switch (act) {
    case BarAction.exit:
      appWindow.close()
      break
    case BarAction.toggleMax:
      appWindow.toggleMaximize()
      break
    case BarAction.mini:
      appWindow.minimize()
      break
  }

}

</script>

<template>
  <div class="flex h-full">
    <div class="menu w-[200px] bg-[#282b3d] p-10" :style="{
      width: !min ? '200px' : '80px'
    }">
      <div class="h-26" data-tauri-drag-region>
        <img :src="logo" alt="logo" width="20" height="20">
        <span>{{ver}}</span>
      </div>
      <a-space direction="vertical" fill>
        <NavigatorItem id="0" :mini="min" icon="i-ic-baseline-dashboard" label="首页"/>
        <NavigatorItem id="0" :mini="min" icon="i-ic-baseline-dashboard" label="首页"/>
        <NavigatorItem id="0" :mini="min" icon="i-ic-baseline-dashboard" label="首页"/>
        <a-button @click="min = !min">toggle</a-button>
      </a-space>
    </div>
    <div class="w-full">
      <div class="w-full h-26 bg-white flex justify-between border-b" data-tauri-drag-region>
        <div></div>
        <div class="flex">
          <div
              @click="doWindowAction(BarAction.mini)"
              class="w-[26px] flex flex-justify-center text-[gray] items-center text-[16px] hover:bg-[#0001]  hover:text-[black] hover:cursor-pointer">
            <div class="i-ic-round-minimize"></div>
          </div>
          <div
              @click="doWindowAction(BarAction.toggleMax)"
              class="w-[26px] flex flex-justify-center text-[gray] items-center text-[16px] hover:bg-[#0001] hover:text-[black] hover:cursor-pointer">
            <div class="i-ic-round-fullscreen"></div>
          </div>
          <div
              @click="doWindowAction(BarAction.exit)"
              class="w-[26px] flex flex-justify-center text-[gray] items-center text-[16px] hover:bg-[#0001] hover:text-[red] hover:cursor-pointer">
            <div class="i-ic-round-close"></div>
          </div>
        </div>

      </div>
      <RouterView/>
    </div>
  </div>
</template>

<style>
#app {
  height: 100vh;
}
</style>

<style scoped>
.menu {
  transition: width 0.168s linear;
}
</style>