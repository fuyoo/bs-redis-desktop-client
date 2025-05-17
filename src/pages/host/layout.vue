<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useReqStore } from '@/stores/req.ts'
import { useI18n } from 'vue-i18n'
const router = useRouter()
const route = useRoute()
const reqStore = useReqStore()
const { t } = useI18n()
const changeTab = async (tab: string) => {
  await router.push({
    path: `/tab/${route.params.id}/main/${tab}`,
    query: route.query,
  })
  console.log(route.path)
}
const tab = computed(() => {
  if (route.path.includes('database')) {
    return 'database'
  } else {
    return 'info'
  }
})
const dbs = shallowRef([])
const fetchDbs = async () => {
  const resp = await reqStore.reqWithHost<string>({
    path: '/cmd',
    data: ['config', 'get', 'databases'],
  })

  dbs.value = new Array(Number(resp.data.split('\n')[1])).fill(0).map((_, i) => ({
    label: `${t('normal.0')}.${i}`,
    value: i,
  }))
}
const selectedDb = ref(Number((route.query.db as string) || 0))
fetchDbs()
const reload = async (v: number) => {
  if (v == Number(route.query.db || 0)) {
    return
  }
  await router.replace({
    path: `/tab/${route.params.id}/main/database`,
    query: {
      ...route.query,
      db: v,
    },
  })
  location.reload()
}
</script>
<template>
  <div class="pt-2">
    <n-tabs
      type="card"
      size="small"
      :default-value="tab"
      :on-update:value="changeTab"
      animated
      pane-wrapper-style="display:none"
    >
      <template #prefix>
        <div class="w-5"></div>
      </template>
      <n-tab-pane name="info" tab="info">
        <template #tab>
          <i class="i-hugeicons:cpu-settings text-5"></i>
        </template>
      </n-tab-pane>
      <n-tab-pane name="database" tab="database">
        <template #tab>
          <i class="i-hugeicons:database text-5"></i>

          <div class="ml-1" v-if="route.query.db || tab == 'database'" @click.stop>
            <n-popselect
              v-model:value="selectedDb"
              :on-update:value="reload"
              scrollable
              :options="dbs"
              trigger="click"
            >
              <span>
                <span></span>{{ t('normal.0') }}.{{ route.query.db || 0 }}
                <i class="i-material-symbols:arrow-drop-down-rounded"></i>
              </span>
            </n-popselect>
          </div>
        </template>
      </n-tab-pane>
    </n-tabs>
  </div>
  <!-- <div class="flex gap-4 h-40px b-b-1px b-b-solid items-center b-b-#eee">
    <div class="ml-5 h-full flex items-center" :class="{ active: tab === 'info' }">
      <q-btn flat round dense @click="changeTab('info')">
        <i class="i-hugeicons:cpu-settings text-5 c"></i>
      </q-btn>
    </div>
    <div class="h-full flex items-center" :class="{ active: tab === 'database' }">
      <q-btn flat round dense @click="changeTab('database')">
        <i class="i-hugeicons:database text-5 c"></i>
      </q-btn>
      <n-dropdown trigger="click" :options="dbs" @select="handleSelect">
        <n-button>点击！</n-button>
      </n-dropdown>
    </div>
  </div> -->
  <div class="w-full bg-white h-[calc(100%-41px)] overflow-auto">
    <router-view />
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
