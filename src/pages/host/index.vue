<script setup lang="ts">
import { useRoute } from 'vue-router'

const route = useRoute()
import CoHostInfo from './components/CoHostInfo.vue'
import { ref } from 'vue'
import { useReqStore } from '@/stores/req'
import CoDatabase from './components/CoDatabase.vue'
const navTab = ref('status')
const reqStore = useReqStore()
const dbsize = ref(16)
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
      <q-space></q-space>
      <q-tabs no-caps dense shrink stretch inline-label narrow-indicator outside-arrows mobile-arrows v-model="navTab">
        <q-tab v-for="i in dbsize" :key="i" :name="`tab-${i}`">
          {{ `db@${i}` }}
        </q-tab>
      </q-tabs>
    </q-toolbar>
    <q-tab-panels v-model="navTab">

      <q-tab-panel name="status" class="q-pa-none">
        <q-scroll-area class="h-[calc(100vh-50px)]">
          <co-host-info></co-host-info>
        </q-scroll-area>
      </q-tab-panel>
      <q-tab-panel v-for="i in dbsize" :key="i" :name="`tab-${i}`" class="q-pa-none">
        <q-scroll-area class="h-[calc(100vh-50px)]">
          <co-database :db="i - 1"></co-database>
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
