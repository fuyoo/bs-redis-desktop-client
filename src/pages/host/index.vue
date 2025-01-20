<script setup lang="ts">
import CoHostInfo from './components/CoHostInfo.vue'
import { ref } from 'vue'
import { useReqStore } from '@/stores/req'
import CoDatabase from './components/CoDatabase.vue'
const navTab = ref('status')
const reqStore = useReqStore()
const dbsize = ref(16)
const tabs = ref(new Set<number>())
const useDb = (db: number) => {
  tabs.value.add(db)
  navTab.value = `tab-${db}`
}
const unUseDb = (db: number) => {
  tabs.value.delete(db)
  if (tabs.value.size == 0) {
    navTab.value = "status"
  } else {
    setTimeout(() => {
      navTab.value = `tab-${[...tabs.value][tabs.value.size - 1]}`
    }, 100)
  }
}
const splitterModel = ref(50)
const doCommand = async () => {
  const resp = await reqStore.reqWithHost<string>({
    path: "/cmd",
    data: JSON.stringify(["config", "get", "databases"]),
  })
  try {
    dbsize.value = Number(resp.data.split("\n")[1] || 16)
  } catch (e) {
    console.log(e)
  }
}
doCommand()
</script>
<template>
  <div>
    <q-toolbar class="text-primary b-b b-b-solid b-b-#eee">
      <q-btn flat round dense @click="navTab = 'status'">
        <i class="i-material-symbols:empty-dashboard text-5"></i>
      </q-btn>
      <q-tabs no-caps dense shrink stretch inline-label narrow-indicator outside-arrows mobile-arrows v-model="navTab">
        <q-tab v-for="i in tabs" :key="i" :name="`tab-${i}`">
          <span class="mr-1">{{ `db@${i}` }}</span>
          <q-btn flat round dense size="xs" @click="unUseDb(i)" icon="close"></q-btn>
        </q-tab>
      </q-tabs>
    </q-toolbar>
    <q-tab-panels v-model="navTab">

      <q-tab-panel name="status" class="q-pa-none">
        <q-splitter v-model="splitterModel">
          <template v-slot:before>
            <q-scroll-area class="h-[calc(100vh-50px)]">
              <co-host-info />
            </q-scroll-area>
          </template>

          <template v-slot:after>
            <q-scroll-area class="h-[calc(100vh-50px)]">
              <div class="flex">
                <div class="flex flex-wrap gap-4 p-4">
                  <q-btn flat dense @click="useDb(db - 1)"
                    class="select-none w-15 h-15 b shadow-lg b-solid b-#eee rounded-lg flex flex-col justify-center items-center text-gray-500"
                    v-for="db in dbsize" :key="db"
                    :class="{ 'b-green': tabs.has(db - 1), 'text-green': tabs.has(db - 1) }">
                    <i class="i-material-symbols:database text-4"></i>
                    <span class="select-none">db@{{ db - 1 }}</span>
                  </q-btn>
                </div>
              </div>

            </q-scroll-area>
          </template>

        </q-splitter>
      </q-tab-panel>
      <q-tab-panel v-for="db in tabs" :key="db" :name="`tab-${db}`" class="q-pa-none">
        <q-scroll-area class="h-[calc(100vh-50px)]">
          <co-database :db="db"></co-database>
        </q-scroll-area>
      </q-tab-panel>
    </q-tab-panels>
  </div>
</template>

<style lang="scss" scoped>
._tab {
  margin: 8px 0;
  height: 50px;
}

._db-active {
  background: #ebebeb;
  width: 70px;
  border-radius: 8px;
  height: 50px;
  padding: 0 8px;
  color: #1976d2;
}
</style>
