<script setup lang="ts">
import useTabStore from '../../store/tabs.ts'
import {nextTick, ref} from 'vue'
import {ConnectionImpl, ConnectionInfo} from '@/database.ts'
import HostForm from '@/views/HostView/HostForm.vue'
import {DrawerMode} from "@/interface.ts";
import db from "@/database"

const tabStore = useTabStore()
const jumpTo = (connection?:ConnectionImpl) => {
  console.log(connection)
  if(!connection || !connection.id) {
    return
  }
  tabStore.addTab({
    id: connection.id,
    path: `/tab/${connection.id}`,
    name: connection.name
  })
}


const mode = ref(DrawerMode.add)
const drawer = ref(false)
const connectionData = ref<ConnectionImpl | undefined>()
const openDrawerFn = (m: DrawerMode, data?: ConnectionImpl) => {
  drawer.value = !drawer.value
  mode.value = m
  connectionData.value = data
}
const formRef = ref<any>()
const handleSelect = async (e: any) => {
  switch (e) {
    case "save":
      await formRef.value?.save()
      break
    case "delete":
      await db.connection.delete(connectionData.value?.id);
      drawer.value = false
      await queryHostList();
      break
    case "connect":
      jumpTo(connectionData.value)
  }
}
nextTick(() => {
  try {
    document.body.removeChild(document.querySelector('._do_first_loading_container')!)
  } catch (e) {
  }
})
const hostList = ref<ConnectionImpl[]>([])
const queryHostList = async () => {
  hostList.value = await db.connection.filter(() => true).toArray()
}
queryHostList()
const onOk = () => {
  drawer.value = false
  queryHostList()
}
</script>

<template>
  <div class="flex">
    <div class="drawer-transition w-full" :class="{'drawer-open':drawer}">
      <div class="flex bg-#E6EBEB h-50px sticky left-0 top-0  justify-between items-center"
           style="z-index: 2">
        <div class="ml-15px text-18px">
          <div class="i-tabler:server text-24px"></div>
          {{ $t('layout.host' as any) }}
        </div>

        <a-button type="primary" class="mr-15px" size="mini" @click="openDrawerFn(DrawerMode.add)">
          <template #icon>
            <div class="i-ic:round-plus"></div>
          </template>
          {{ $t('layout.add' as any) }}
        </a-button>
      </div>
      <div class="flex flex-wrap _main p-25px">
        <a-empty description="noting at here." v-show="hostList.length == 0"></a-empty>
        <div @dblclick="jumpTo(host)"
             class="select-none cursor-pointer relative flex p-10 w-280px rounded-lg items-center bg-white _host_items"
             v-for="(host,i) in hostList" :key="i">
          <div class="p-5 rounded-lg  bg-#282B3D text-white">
            <div class="i-tabler:server-bolt text-26px"></div>
          </div>
          <div class="flex ml-10px flex-col text-overflow justify-between">
            <div class="font-500 w-230px pb-3px text-overflow">
              {{ host.name }}
            </div>
            <div class="text-gray w-230px text-overflow">{{ host.cluster ? 'cluster' : `${host.node[0].host}${host.node[0].password ? ',password':''}` }}</div>
          </div>
          <div @dblclick.stop @click.stop="openDrawerFn(DrawerMode.edit,host)"
               class="rounded-md w-20px h-20px absolute right-10px top-14px p-4 _edit bg-amber justify-center items-center hidden">
            <div class="i-ic-round-edit"></div>
          </div>
        </div>
      </div>
    </div>
    <div class="drawer-transition w-0 overflow-hidden" :class="{drawer:drawer}">
      <div class="flex bg-#E6EBEB h-50px sticky left-0 top-0  justify-between items-center"
           style="z-index: 2">
            <span class="ml-15px text-14px  flex-shrink-0">
                <div class="i-tabler:server text-20px"></div>
                {{ mode == DrawerMode.add ? $t('layout.add' as any) : $t('layout.edit' as any) }}</span>

        <a-space>
          <div>
            <a-dropdown @select="handleSelect" position="br" v-if="mode == DrawerMode.edit">
              <div class="i-ic:round-more-horiz text-20px"></div>
              <template #content>
                <a-doption value="connect">
                  <template #icon>
                    <div class="i-ic:twotone-cast-connected text-18px"></div>
                  </template>
                  {{ $t('layout.hostForm.connect') }}
                </a-doption>
                <a-doption value="save">
                  <template #icon>
                    <div class="i-ic:round-save text-18px"></div>
                  </template>
                  {{ $t('layout.hostForm.save') }}
                </a-doption>
                <a-doption value="delete">
                  <template #icon>
                    <div class="i-ic:round-delete text-18px"></div>
                  </template>
                  {{ $t('layout.hostForm.delete') }}
                </a-doption>
              </template>
            </a-dropdown>
          </div>
          <div class="i-ic:round-close text-20px" @click="drawer = false"></div>
          <div class="w-5px"></div>
        </a-space>
      </div>
      <div class="h-400px overflow-auto _form_ctx">

        <HostForm :mode="mode" :data="connectionData" @ok="onOk" ref="formRef"></HostForm>
      </div>
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

._form_ctx {
  height: calc(100% - 50px);
  overflow: auto;

  &::-webkit-scrollbar {
    display: none;
  }
}

</style>