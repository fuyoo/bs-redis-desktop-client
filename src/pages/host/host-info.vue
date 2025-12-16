<script lang="ts" setup>
import { computed, ref } from 'vue'

import { useReqStore } from '@/stores/req.ts'
import { useRoute } from 'vue-router'
import { useI18n } from "vue-i18n"
const { t } = useI18n()
const reqStore = useReqStore()
const config = ref<Record<string, Record<string, any>>>({})
const fetchInfo = async () => {
  const data = await reqStore.reqWithHost<string>({
    path: '/info',
    data: '',
  })
  const cfg = {} as Record<string, any>
  data.data
    .split('#')
    .filter((e) => !!e)
    .map((e) => e.trimStart())
    .map((e) => e.split('\r\n').filter((e) => !!e))
    .forEach((per) => {
      const key = per[0]
      cfg[key] = []
      for (const i of per) {
        if (i != key) {
          cfg[key].push(i)
        }
      }
    })
  config.value = cfg
}
fetchInfo()
const helper = (k: string, section: string) => {
  try {
    const v = config.value[section].filter((e: string) => e.indexOf(k) != -1)
    if (v.length > 0) {
      return v[0].split(':')[1]
    } else {
      return 0
    }
  } catch (e) {
    return 0
  }
}

const columns = [
  { title: 'db', key: 'db', },
  { title: 'keys', key: 'keys', },
  { title: 'expires', key: 'expires', },
  { title: 'avg_ttl', key: 'avg_ttl', },
  { title: 'subexpiry', key: 'subexpiry', },
] as any
const rows = computed(() => {
  const space = config.value['Keyspace']?.map((item: string) => {
    const obj = {} as {
      db: string
      keys: string
      expires: string
      avg_ttl: string
      subexpiry: string
      [K: string]: string
    }
    const v = item.split(':')
    obj.db = v[0]
    v[1].split(',').forEach((item: string) => {
      const v: string[] = item.split('=')
      obj[v[0]] = v[1]
    })
    return obj
  })
  console.log(space)
  return space || []
})
</script>
<template>
  <n-scrollbar class="h-full w-full absolute left-0 top-0">
    <div class="flex gap-4 p-4 flex-wrap">
      <n-descriptions content-class="whitespace-nowrap" label-class="whitespace-nowrap" size="small" class="flex-1"
        :columns="1" label-placement="left" bordered :title="t('hostInfo[1]')">
        <n-descriptions-item label="redis_version">
          {{ helper('redis_version', 'Server') }}
        </n-descriptions-item>
        <n-descriptions-item label="uptime_in_days">
          {{ helper('uptime_in_days', 'Server') }}
        </n-descriptions-item>
        <n-descriptions-item label="executable">
          {{ helper('uptime_in_days', 'Server') }}
        </n-descriptions-item>
        <n-descriptions-item label="config_file">
          {{ helper('config_file', 'Server') }}
        </n-descriptions-item>
      </n-descriptions>
      <n-descriptions content-class="whitespace-nowrap" label-class="whitespace-nowrap" size="small" class="flex-1"
        :columns="1" label-placement="left" bordered :title="t('hostInfo[0]')">
        <n-descriptions-item label="Used Memory">
          {{ helper('used_memory_human', 'Memory') }}
        </n-descriptions-item>
        <n-descriptions-item label="Memory Peak">
          {{ helper('used_memory_peak_human', 'Memory') }}
        </n-descriptions-item>
        <n-descriptions-item label="Total System Memory">
          {{ helper('total_system_memory_human', 'Memory') }}
        </n-descriptions-item>
        <n-descriptions-item label="Used Lua Memory">
          {{ helper('used_memory_lua_human', 'Memory') }}
        </n-descriptions-item>
      </n-descriptions>
      <n-descriptions content-class="whitespace-nowrap" label-class="whitespace-nowrap" size="small" class="flex-1"
        :columns="1" label-placement="left" bordered :title="t('hostInfo[2]')">
        <n-descriptions-item label="Connected Clients">
          {{ helper('used_memory_human', 'Memory') }}
        </n-descriptions-item>
        <n-descriptions-item label="Total Connections Received">
          {{ helper('total_connections_received', 'Stats') }}
        </n-descriptions-item>
        <n-descriptions-item label="Total Commands Processed">
          {{ helper('total_commands_processed', 'Stats') }}
        </n-descriptions-item>
        <n-descriptions-item label="Rejected Connections">
          {{ helper('rejected_connections', 'Status') }}
        </n-descriptions-item>
      </n-descriptions>
    </div>
    <div class="p-4">
      <n-h4 prefix="bar">{{ t('hostInfo[4]') }}</n-h4>
      <n-data-table size="small" class="w-full" :columns="columns" :data="rows"></n-data-table>
    </div>
  </n-scrollbar>
</template>

<style lang="scss" scope>
.renderjson a {
  text-decoration: none;
}
</style>
