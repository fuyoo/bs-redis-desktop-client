<script setup lang="ts">
import { computed, ref, shallowRef } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useReqStore } from '@/stores/req.ts'
import { useI18n } from 'vue-i18n'
import { db, type ConnectionHost } from '@/db'
const router = useRouter()
const route = useRoute()
const reqStore = useReqStore()
const { t } = useI18n()
const changeTab = async (tab: string) => {
  await router.push({
    path: `/tab/${route.params.id}/main/${tab}`,
    query: route.query
  })
  console.log(route.path)
}
const tab = computed(() => {
  if (route.path.includes('database')) {
    return 'database'
  }
  else {
    return 'info'
  }
})
const selectedDb = ref(Number((route.query.db as string) || -1))
const dbs = shallowRef<Record<string, any>[]>([])
const fetchDbs = async () => {
  if (selectedDb.value < 0) {
    // fetch host info
    const inf = await db.hosts.get(parseInt(route.params.id as string))
    // set default db
    selectedDb.value = Number(inf?.node?.[0].db || 0)
  }

  const resp = await reqStore.reqWithHost<string>({
    path: '/cmd',
    data: ['config', 'get', 'databases']
  })

  dbs.value = new Array(Number(resp.data.split('\n')[1])).fill(0).map((_, i) => ({
    label: `${t('normal.0')}.${i}`,
    value: i
  }))
}
fetchDbs()
const reload = async (v: number) => {
  if (v == Number(route.query.db || selectedDb.value)) {
    return
  }
  await router.replace({
    path: `/tab/${route.params.id}/main/database`,
    query: {
      ...route.query,
      db: v
    }
  })
  location.reload()
}
</script>
<template>
  <div class="absolute left-0 top-1px w-full h-full flex flex-col">
    <div class="h-2 shrink-0"></div>
    <n-tabs class="shrink-0" type="card" size="small" :default-value="tab" :on-update:value="changeTab" animated
      pane-wrapper-style="display:none">
      <template #prefix>
        <div class="w-5"></div>
      </template>
      <n-tab name="info" tab="info">
        <i class="i-hugeicons:cpu-settings text-5"></i>
      </n-tab>
      <n-tab name="database" tab="database">
        <i class="i-hugeicons:database text-5"></i>
        <div class="ml-1" v-if="route.query.db || tab == 'database'" @click.stop>
          <n-popselect v-model:value="selectedDb" :on-update:value="reload" scrollable :options="dbs" trigger="click">
            <span>
              <span></span>{{ t('normal.0') }}.{{ route.query.db || selectedDb }}
              <i class="i-material-symbols:arrow-drop-down-rounded"></i>
            </span>
          </n-popselect>
        </div>
      </n-tab>
    </n-tabs>
    <div class="flex flex-1 relative">
      <router-view />
    </div>
  </div>
</template>

<style lang="scss" scoped>
._tab {
  margin: 8px 0;
  height: 50px;
}

.active {
  position: relative;

  &::after {
    content: '';
    position: absolute;
    left: 0;
    bottom: 0;
    height: 2px;
    width: 100%;
    border-radius: 2px;
    background: var(--q-primary);
    color: var(--q-primary);
  }

  .c {
    color: var(--q-primary);
  }
}
</style>
