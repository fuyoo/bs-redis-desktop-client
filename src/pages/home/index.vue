<template>
  <q-page class="p-4">
    <div class="flex gap-4">
      <div v-for="i in hosts" :key="i.id" @click="changeTab(i)"
        class="relative px-4 h-15 shadow-lg b b-solid b-#0001 flex justify-center items-center rounded cursor-pointer active:opacity-50">
        <div class="bg-#000 text-white rounded flex justify-center items-center text-5 w-8 h-8 mr-3">
          <i class="i-mdi:server-network"></i>
        </div>
        <div class="flex-1 flex flex-col mr-2 justify-center items-start select-none">
          <span class="text-4 text-#333 inline-block max-w-50">{{ i.name }}</span>
          <span class="text-3 text-#0006">{{ i.cluster ? 'cluster' : 'stand-alone' }}</span>
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
import { useTabStore } from '@/stores/tab';
import { Dialog } from 'quasar';
import CoHostForm from "./components/CoHostForm.vue"
import { liveQuery, type Observable } from "dexie";
import { useObservable } from "@vueuse/rxjs";
import { db, type ConnectionHost } from "@/db";
import { watch, type Ref } from 'vue';
const tab = useTabStore()
const changeTab = (i: ConnectionHost) => {
  tab.change({ id: i.id!.toString(), name: i.name })
}
const hosts: Ref<ConnectionHost[]> = useObservable(liveQuery(() => db.hosts.toArray()) as any)
const addFn = () => {
  Dialog.create({
    component: CoHostForm,

    // props forwarded to your custom component
    componentProps: {
      text: 'something',
      persistent: true,
      // ...more..props...
    }
  }).onOk(() => {
    console.log('OK')
  }).onCancel(() => {
    console.log('Cancel')
  }).onDismiss(() => {
    console.log('Called on OK or Cancel')
  })
}
</script>
