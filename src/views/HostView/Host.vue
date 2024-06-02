<script setup lang="ts">
import useTabStore from '../../store/tabs.ts'
import {ref} from 'vue'
import {ConnectionImpl} from '@/database.ts'
import HostForm from '@/views/HostView/HostForm.vue'

const tabStore = useTabStore()
const jumpTo = () => {
  const id = Math.ceil(Math.random() * 10e9)
  tabStore.addTab({
    id: id,
    path: `/tab/${id}`,
    name: '' + id
  })
}

enum DrawerMode {
  edit,
  add
}

const mode = ref(DrawerMode.add)
const drawer = ref(false)
const openDrawerFn = (m: DrawerMode, data?: ConnectionImpl) => {
  console.log(m, data)
  drawer.value = true
  mode.value = m
}
const handleSelect = ()=>{}
</script>

<template>
  <div class="flex">
    <div class="drawer-transition w-full" :class="{'drawer-open':drawer}">
      <div class="flex bg-#E6EBEB h-50px sticky left-0 top-0  justify-between items-center"
           style="z-index: 2">
            <span class="ml-15px text-18px">
                <div class="i-tabler:server text-24px"></div>
                {{ $t('layout.host' as any) }}</span>

        <a-button type="primary" class="mr-15px" size="mini" @click="openDrawerFn(DrawerMode.add)">
          <template #icon>
            <div class="i-ic:round-plus"></div>
          </template>
          {{ $t('layout.add' as any) }}
        </a-button>
      </div>
      <div class="flex flex-wrap _main p-25px">
        <div @dblclick="jumpTo()"
             class="select-none cursor-pointer relative flex p-10 w-280px rounded-lg items-center bg-white _host_items"
             v-for="i in 9" :key="i">
          <div class="p-5 rounded-lg  bg-#282B3D text-white">
            <div class="i-tabler:server-bolt text-26px"></div>
          </div>
          <div class="flex ml-10px flex-col text-overflow justify-between">
            <div class="font-500 w-230px pb-3px text-overflow">10.35.11.21511111111111asdfasdfasdfasdfasdfasdf1111
            </div>
            <div class="text-gray w-230px text-overflow">password,cluster</div>
          </div>
          <div @click="openDrawerFn(DrawerMode.edit)"
               class="rounded-md w-20px h-20px absolute right-10px top-14px p-4 _edit bg-amber justify-center items-center hidden">
            <div class="i-ic-round-edit"></div>
          </div>
        </div>
      </div>
    </div>
    <div class="drawer-transition w-0 overflow-hidden" :class="{drawer:drawer}">
      <div class="flex bg-#E6EBEB h-50px sticky left-0 top-0  justify-between items-center"
           style="z-index: 2">
            <span class="ml-15px text-14px">
                <div class="i-tabler:server text-20px"></div>
                {{ mode == DrawerMode.add ? $t('layout.add' as any) : $t('layout.edit' as any) }}</span>

        <a-space>
          <div>
            <a-dropdown @select="handleSelect" position="br" v-if="mode == DrawerMode.edit">
              <div class="i-ic:round-more-horiz text-20px"></div>
              <template #content>
                <a-doption>
                  <template #icon>
                    <div class="i-ic:twotone-cast-connected text-18px"></div>
                  </template>
                  Connect
                </a-doption>
                <a-doption>
                  <template #icon>
                    <div class="i-ic:round-save text-18px"></div>
                  </template>
                  Save
                </a-doption>
                <a-doption>
                  <template #icon>
                    <div class="i-ic:round-delete text-18px"></div>
                  </template>
                  Delete
                </a-doption>
              </template>
            </a-dropdown></div>
          <div class="i-ic:round-arrow-forward text-20px" @click="drawer = false"></div>
          <div class="w-5px"></div>
        </a-space>
      </div>
      <HostForm></HostForm>
    </div>
  </div>
</template>

<style scoped lang="scss">
$drawer-width: 330px;
._main {
  gap: 25px;
}

._host_items {
  box-shadow: 1px 2px 4px #0001;

  &:hover > ._edit {
    display: flex;
  }
}

.drawer-transition {
  transition: width ease-in-out 0.168s;
}

.overflow-hidden {
  overflow: hidden;
}

.drawer-open {
  width: calc(100% - $drawer-width);
}

.drawer {
  width: $drawer-width;
  height: calc(100vh - 30px);
  background: #fff;
  overflow: auto;
  border-left: 1px solid #0002;
  position: sticky;
  top: 0;
  right: 0;

  &::-webkit-scrollbar {
    width: 4px;
    height: 4px;
  }

  &::-webkit-scrollbar-thumb {
    background: #000;
    border-radius: 2px;
  }
}


</style>