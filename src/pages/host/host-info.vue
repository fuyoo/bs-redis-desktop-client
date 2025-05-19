<script lang="ts" setup>
import { computed, ref } from 'vue'

import { useReqStore } from '@/stores/req.ts'
import { showHostConfigureDetail } from '@/tools'
import { useRoute } from 'vue-router'
const route = useRoute()
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
  { name: 'db', label: 'db', field: 'db', align: 'left' },
  { name: 'keys', label: 'keys', field: 'keys', align: 'left' },
  { name: 'expires', label: 'expires', field: 'expires', align: 'left' },
  { name: 'avg_ttl', label: 'avg_ttl', field: 'avg_ttl', align: 'left' },
  { name: 'subexpiry', label: 'subexpiry', field: 'subexpiry', align: 'left' },
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
  return space || []
})
</script>
<template>
  <n-scrollbar class="h-full w-full" trigger="none">
    <div class="w-full flex gap-4 p-4">
      <q-card flat class="flex-1 b b-solid b-#eee">
        <q-card-section class="flex justify-between items-center">
          <div class="text-h6">{{ $t('hostInfo[1]') }}</div>
          <q-btn
            icon="info"
            dense
            flat
            rounded
            @click="showHostConfigureDetail(route.params.id as string)"
          ></q-btn>
        </q-card-section>
        <q-separator />
        <q-card-actions vertical>
          <q-chip>redis_version: {{ helper('redis_version', 'Server') }}</q-chip>
          <q-chip>uptime_in_days: {{ helper('uptime_in_days', 'Server') }}</q-chip>
          <q-chip>executable: {{ helper('executable', 'Server') }}</q-chip>
          <q-chip>config_file:{{ helper('config_file', 'Server') }} </q-chip>
        </q-card-actions>
      </q-card>
      <q-card flat class="flex-1 b b-solid b-#eee">
        <q-card-section>
          <div class="text-h6">{{ $t('hostInfo[0]') }}</div>
        </q-card-section>
        <q-separator />
        <q-card-actions vertical>
          <q-chip>Used Memory: {{ helper('used_memory_human', 'Memory') }}</q-chip>
          <q-chip>Memory Peak: {{ helper('used_memory_peak_human', 'Memory') }}</q-chip>
          <q-chip>Total System Memory: {{ helper('total_system_memory_human', 'Memory') }}</q-chip>
          <q-chip>Used Lua Memory :{{ helper('used_memory_lua_human', 'Memory') }} </q-chip>
        </q-card-actions>
      </q-card>
      <q-card flat class="flex-1 b b-solid b-#eee">
        <q-card-section>
          <div class="text-h6">{{ $t('hostInfo[2]') }}</div>
        </q-card-section>
        <q-separator />
        <q-card-actions vertical>
          <q-chip>Connected Clients :{{ helper('connected_clients', 'Clients') }} </q-chip>
          <q-chip
            >Total Connections Received: {{ helper('total_connections_received', 'Stats') }}</q-chip
          >
          <q-chip
            >Total Commands Processed: {{ helper('total_commands_processed', 'Stats') }}</q-chip
          >
          <q-chip>Rejected Connections: {{ helper('rejected_connections', 'Status') }}</q-chip>
        </q-card-actions>
      </q-card>
      <q-card flat class="w-full b b-solid b-#eee">
        <q-card-section>
          <div class="text-h6">{{ $t('hostInfo[4]') }}</div>
        </q-card-section>
        <q-table flat class="w-full b b-solid b-#eee" :columns="columns" :rows="rows"></q-table>
      </q-card>
    </div>
  </n-scrollbar>
</template>

<style lang="scss" scope>
.renderjson a {
  text-decoration: none;
}
</style>
