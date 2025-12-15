<template>
  <div class="flex absolute left-0 top-0 w-full">
    <div class="w-188px pl-2 pt-2 pb-2 h-full">
      <n-button block @click="newConnection">新链接</n-button>
      <div class="h-2"></div>
      <n-tabs tab-class="relative h-9" placement="left" size="small" style="height: calc(100vh - 100px)" type="card"
        v-model:value="tab">
        <n-tab v-for="item in hosts" :name="`${item.id}`" :title="item.name" class="w-180px">
          <template #default>
            <div class="absolute left-0 right-0 h-full flex items-center indent-1em">
              <n-ellipsis>{{ item.name }}</n-ellipsis>
            </div>
          </template>
        </n-tab>
      </n-tabs>
    </div>
    <div class="flex-1 relative">
      <n-scrollbar class="w-full h-full absolute left-0 top-0" content-class="flex justify-center items-start">
        <div class="p-4 m-5 w-600px rounded-lg">
          <CoHostForm :data="tabData"></CoHostForm>
        </div>
      </n-scrollbar>
    </div>
  </div>
</template>

<script setup lang="ts">
import CoHostForm from './components/CoHostForm.vue'
import { useObservable } from '@vueuse/rxjs'
import { type ConnectionHost, db } from '@/db.ts'
import { liveQuery } from 'dexie'
const tab = ref('')
const hosts: Ref<ConnectionHost[]> = useObservable(liveQuery(() => db.hosts.toArray()) as any)
const tabData = ref()
watch(
  () => hosts.value,
  (val) => {
    const v = val.find((e) => e.id?.toString() == tab.value)
    if (!v) tab.value = 'new'
  },
)
watch(
  () => tab.value,
  (val) => {
    const v = toRaw(hosts.value.find((e) => e.id?.toString() == val))
    if (v) tabData.value = toRaw(v)
    else tabData.value = undefined
  },
)
const newConnection = () => {
  tab.value = 'new'
}

</script>
