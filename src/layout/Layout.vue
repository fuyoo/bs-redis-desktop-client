<script setup lang="ts">
import {appWindow} from '@tauri-apps/api/window'
import NavigatorItem from '../components/NavigatorItem.vue'
import logo from '../assets/logo.png'
import { ref} from 'vue'
import useTabStore from '../store/tabs'

const tabStore = useTabStore()
const min = ref(false)
const ver = import.meta.env.VITE_VERSION.version
const ScrollerRef = ref<any>()
tabStore.setScroller(() => {
  ScrollerRef.value?.scrollTop(tabStore.list.length * 80)
})

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

const okFn = (id: string | number) => tabStore.changeTab(id as number)

const closeFn = (id: string | number) => tabStore.delTab(id as number)
const isMacos = navigator.userAgent.indexOf("Mac") > -1
console.log(isMacos)
</script>

<template>
  <div class="flex h-full">
    <div class="menu w-[200px] bg-[#282b3d] " :style="{
      width: !min ? '200px' : '70px'
    }">
      <div class="h-26 select-none flex items-center  p-10" data-tauri-drag-region :class="{
        ' justify-center':min,
        'justify-start':!min,
        'flex-shrink': 0,
      }" :style="{
        paddingTop: isMacos ? '35px' : '0'}">
        <img data-tauri-drag-region :src="logo" alt="logo" width="30" height="30" class="mr-1">
        <div v-if="!min" class="text-gray w-120px text-ellipsis text-nowrap"
             style="white-space: nowrap;overflow: hidden" data-tauri-drag-region>BS <small
            data-tauri-drag-region>v{{ ver }}</small></div>
      </div>
      <div class="h-10px"></div>
      <div class="px-[10px]">
        <a-space direction="vertical" fill>
          <NavigatorItem :id="-1" :mini="min" :active="-1 === tabStore.activeTab?.id" icon="i-mdi:server-network"
                         :closeable="false"
                         :label="$t('layout.host' as any)" @ok="okFn"/>
          <NavigatorItem :id="-2" :mini="min" :active="-2 === tabStore.activeTab?.id" icon="i-ic:round-settings"
                         :closeable="false"
                         :label="$t('layout.settings' as any)" @ok="okFn"/>
        </a-space>
      </div>
      <div class="_tabscroller py-0px overflow-auto" ref="ScrollerRef" :style="{height: `calc(100% - ${isMacos ? 182 + 35 : 182}px)`}">
        <a-space direction="vertical" fill class="px-10px py-8px">

          <NavigatorItem v-for="item in tabStore.list" :id="item.id" :mini="min" tab
                         :active="item.id === tabStore.activeTab?.id"
                         icon="i-tabler:server-bolt" :label="item.name" @ok="okFn" @close="closeFn"/>
        </a-space>
      </div>
      <div class="h-42px flex items-center px-20px" :class="{
        ' justify-center':min,
        'justify-end':!min
      }">
        <div @click="min = !min" class="text-26px text-white">
          <div class="i-tabler:layout-sidebar-left-collapse-filled" v-if="!min"></div>
          <div class="i-tabler:layout-sidebar-right-collapse-filled" v-else></div>
        </div>
      </div>
    </div>
    <div class="w-full">
      <div class="w-full _act_bar bg-#EDF1F2 flex justify-between border-b" data-tauri-drag-region>
        <div style="height: 30px" class="select-none" data-tauri-drag-region></div>
        <div class="flex" v-if="!isMacos">
          <div
              @click="doWindowAction(BarAction.mini)"
              class="w-[30px] flex flex-justify-center text-[gray] items-center text-[16px] hover:bg-[#0001]  hover:text-[black] hover:cursor-pointer">
            <div class="i-ic-round-minimize"></div>
          </div>
          <div
              @click="doWindowAction(BarAction.toggleMax)"
              class="w-[30px] flex flex-justify-center text-[gray] items-center text-[16px] hover:bg-[#0001] hover:text-[black] hover:cursor-pointer">
            <div class="i-ic-round-fullscreen"></div>
          </div>
          <div
              @click="doWindowAction(BarAction.exit)"
              class="w-[30px] flex flex-justify-center text-[gray] items-center text-[16px] hover:bg-[#0001] hover:text-[red] hover:cursor-pointer">
            <div class="i-ic-round-close"></div>
          </div>
        </div>
      </div>
      <div class="bg-#EDF1F2 _ctx_scroller">
        <router-view v-slot="{ Component }">
          <transition>
            <keep-alive>
              <component :is="Component"/>
            </keep-alive>
          </transition>
        </router-view>
      </div>
    </div>
  </div>
</template>

<style>
#app {
  height: 100vh;
}
</style>

<style lang="scss" scoped>
$act-bar-height: 30px;
.menu {
  transition: width 0.168s linear;
  *{
    cursor: default;
    user-select: none;
  }
}

._act_bar {
  height: $act-bar-height;
}

._tabscroller {
  height: calc(100vh - 192px);

  &::-webkit-scrollbar{
    width: 0;
    height: 0;
  }
}

._ctx_scroller {
  height: calc(100vh - $act-bar-height);
  overflow: auto;

  &::-webkit-scrollbar {
    width: 0px;
    height: 0px;
  }

  &::-webkit-scrollbar-thumb {
    background: #0004;
    border-radius: 4px;
  }
}
</style>