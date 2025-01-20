<script lang="ts" setup>
import { onMounted, ref, shallowRef } from 'vue'
import JSONFormatter from 'json-formatter-js'
import { useReqStore } from '@/stores/req'

const ctxRef = shallowRef<HTMLElement | undefined>()
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
  setTimeout(() => {
    fetchInfo()
  }, 3000)
}
onMounted(async () => {
  await fetchInfo()

})
const helper = (k: string, section: string) => {
  try {
    const v = config.value[section].filter((e: string) => e.indexOf(k) != -1);
    if (v.length > 0) {
      return v[0].split(':')[1]
    } else {
      return 0
    }
  } catch (e) {
    return 0
  }
}
let flag = false
const detailFn = () => {
  if (!flag) {
    const formatter = new JSONFormatter(config.value)
    ctxRef.value!.innerHTML = ''
    ctxRef.value?.appendChild(formatter.render())
  } else {
    ctxRef.value!.innerHTML = ''
  }
  flag = !flag
}
</script>
<template>
  <div class="flex gap-4 p-4">
    <q-card flat class="flex-1 b b-solid b-#eee">
      <q-card-section>
        <div class="text-h6">Server</div>
      </q-card-section>
      <q-separator />
      <q-card-actions vertical>
        <q-chip>redis_version: {{ helper("redis_version", "Server") }}</q-chip>
        <q-chip>uptime_in_days: {{ helper("uptime_in_days", "Server") }}</q-chip>
        <q-chip>executable: {{ helper("executable", "Server") }}</q-chip>
        <q-chip>config_file:{{ helper("config_file", "Server") }} </q-chip>
      </q-card-actions>
    </q-card>
    <q-card flat class="flex-1 b b-solid b-#eee">
      <q-card-section>
        <div class="text-h6">Memory</div>
      </q-card-section>
      <q-separator />
      <q-card-actions vertical>
        <q-chip>Used Memory: {{ helper("used_memory_human", "Memory") }}</q-chip>
        <q-chip>Memory Peak: {{ helper("used_memory_peak_human", "Memory") }}</q-chip>
        <q-chip>Total System Memory: {{ helper("total_system_memory_human", "Memory") }}</q-chip>
        <q-chip>Used Lua Memory :{{ helper("used_memory_lua_human", "Memory") }} </q-chip>
      </q-card-actions>
    </q-card>
    <q-card flat class="flex-1 b b-solid b-#eee">
      <q-card-section>
        <div class="text-h6">Stats</div>
      </q-card-section>
      <q-separator />
      <q-card-actions vertical>
        <q-chip>Connecting Clients :{{ helper("connected_clitens", "Stats") }} </q-chip>
        <q-chip>Total Connections Received: {{ helper("total_connections_received", "Stats") }}</q-chip>
        <q-chip>Total Commands Processed: {{ helper("total_commands_processed", "Stats") }}</q-chip>
        <q-chip>Rejected Connections: {{ helper("rejected_connections", "Status") }}</q-chip>
      </q-card-actions>
    </q-card>
    <div class="p-4 z-1 realative text-4.5" ref="ctxRef"></div>
  </div>
</template>

<style lang="scss" scope>
.renderjson a {
  text-decoration: none;
}
</style>
