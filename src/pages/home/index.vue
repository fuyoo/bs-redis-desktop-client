<template>
  <q-page class="p-4">
    <div class="flex gap-4">
      <div :ref="`tab-${i.id}`" v-for="i in hosts" :key="i.id" @click="changeTab(i)"
        class="_i-c relative px-4 h-15 shadow-lg b b-solid b-#0001 flex justify-center items-center rounded cursor-pointer"
        :class="{
          loading: loadingStack.has(i.id || -1),
          offline: offlineHosts.has(i.id || -1),
          connected: connected.includes((i.id || -1).toString()),
        }" :title="i.name">
        <div class="bg-#000 text-white rounded flex justify-center items-center text-5 w-8 h-8 mr-3 disconnect">
          <i class="i-mdi:server-network def"></i>
          <i class="i-mdi:lan-disconnect off"></i>
          <i class="i-eos-icons:loading connecting"></i>
        </div>
        <div class="flex-1 flex flex-col mr-2 justify-center items-start select-none">
          <span class="text-4 text-#333 inline-block max-w-50 text-nowrap text-ellipsis overflow-hidden">{{ i.name
            }}</span>
          <span class="text-3 text-#0006">{{ i.cluster ? 'cluster' : 'stand-alone' }}</span>
        </div>
        <q-menu touch-position context-menu>
          <q-list class="min-w-25">
            <q-item clickable v-close-popup @click="modifyFn(i)">
              <q-item-section>
                <div>
                  <i class="i-ic:round-edit mr-2 text-4"></i>
                  <span>{{ $t('actions[3]') }}</span>
                </div>
              </q-item-section>
            </q-item>
            <q-separator />
            <q-item clickable v-close-popup @click="delFn(i.id!)">
              <q-item-section class="flex">
                <div>
                  <i class="i-ic:round-delete mr-2 text-4"></i>
                  <span>{{ $t('actions[2]') }}</span>
                </div>
              </q-item-section>
            </q-item>
          </q-list>
        </q-menu>
        <div @click.stop
          class="hidden _h-s w-5 h-5 rounded b b-#0001 b-solid shadow-lg absolute right-0.5 top-0.5 flex justify-center items-center">
          <i class="i-ic:round-more-horiz"></i>
        </div>
      </div>
      <div @click="addFn"
        class="w-15 h-15 shadow-lg b b-solid b-#0001 flex justify-center items-center text-10 rounded cursor-pointer active:opacity-50">
        <i class="i-ic:round-plus"></i>
      </div>
    </div>
  </q-page>
</template>

<script setup lang="ts">
import { useTabStore } from '@/stores/tab'
import { Dialog } from 'quasar'
import CoHostForm from './components/CoHostForm.vue'
import { liveQuery, type Observable } from 'dexie'
import { useObservable } from '@vueuse/rxjs'
import { db, type ConnectionHost } from '@/db'
import { type Ref, computed, reactive } from 'vue'
import { request } from '@/api'
const tab = useTabStore()
const offlineHosts = reactive<Set<number>>(new Set())
const loadingStack = reactive<Set<number>>(new Set())
const connected = computed(() => {
  return tab.tabList.map((item) => item.id)
})
const changeTab = async (i: ConnectionHost) => {
  // reset offline status
  offlineHosts.delete(i.id!)
  // if is loading, we don't need to request again
  if (loadingStack.has(i.id!)) {
    return
  }
  // flag. one's loading status
  loadingStack.add(i.id!)
  // it is response for checking online.
  const { code } = await request({
    path: `/status`,
    connectionInfo: i,
    data: '',
  })
  // code 0 represent is online.
  if (code !== 0) {
    offlineHosts.add(i.id!)
  } else {
    offlineHosts.delete(i.id!)
  }
  // when loading complete, we should remove it from loading stack
  loadingStack.delete(i.id!)
  // when code is 0, we should open a new tab(webview).
  if (code === 0) {
    tab.change({ id: i.id!.toString(), name: i.name })
  }
}
const hosts: Ref<ConnectionHost[]> = useObservable(liveQuery(() => db.hosts.toArray()) as any)
const addFn = () => {
  Dialog.create({
    component: CoHostForm,
  }).onOk((ret: ConnectionHost) => changeTab(ret))
}
const delFn = async (id: number) => {
  await db.hosts.delete(id)
}

const modifyFn = (ch: ConnectionHost) => {
  Dialog.create({
    component: CoHostForm,
    // props forwarded to your custom component
    componentProps: {
      data: ch,
    },
  }).onOk((ret: ConnectionHost) => changeTab(ret))
}
</script>

<style lang="scss" scoped>
@import '@/css/quasar.variables.scss';

.off {
  display: none;
}

.connecting {
  display: none;
}

.offline {
  border-color: $negative;

  .disconnect {
    background-color: $negative;
  }

  .def {
    display: none;
  }

  .off {
    display: inline;
  }
}

.loading {
  .def {
    display: none;
  }

  .off {
    display: none;
  }

  .connecting {
    display: inline;
  }
}

._i-c {
  ._h-s {
    transition: 0.168s all ease-in-out;
    opacity: 0;
  }

  &:hover ._h-s {
    opacity: 1;
  }
}

.connected {
  border-color: $positive;

  * {
    color: $positive;
  }

  &::before {
    content: '';
    display: block;
    width: 10px;
    height: 10px;
    position: absolute;
    left: 5px;
    top: 5px;
    border-radius: 50%;
    background-color: $positive;
  }
}
</style>
