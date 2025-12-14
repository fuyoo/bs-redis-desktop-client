<script setup lang="ts">
import { useReqStore } from '@/stores/req.ts'
import { onBeforeUnmount, reactive } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'
import {dataToHuman} from '@/tools'

const route = useRoute()
const { t } = useI18n()
const prop = defineProps<{ type?: string }>()

const vSizeModel = defineModel('size')
const reqStore = useReqStore()
const baseInfo = reactive({
  memory: '',
  pttl: '',
})

const trans = (k:string | string[]) => atob(k as string)
let timer = -1 as any
const fetchInfo = async () => {
  // clear interval, prevention memory leak
  clearInterval(timer)
  const mem = await reqStore.reqWithHost<string>({
    path: '/cmd',
    data: ['memory', 'usage', trans(route.params.key)],
  })
  baseInfo.memory = mem.data
  vSizeModel.value = mem.data
  const pttl = await reqStore.reqWithHost<string>({
    path: '/cmd',
    data: ['pttl',  trans(route.params.key)],
  })
  baseInfo.pttl = pttl.data
  if (Number(pttl.data) > 1000) {
    timer = setInterval(async () => {
      baseInfo.pttl = (Number(baseInfo.pttl) - 1000).toString()
    }, 1000)
  } else {
    clearInterval(timer)
  }
}
fetchInfo()

const formatMilliseconds = (v: string): string => {
  const ms = Number(v)
  if (ms < 0) {
    return t('timeFormat[5]')
  }
  const millisecondsInSecond = 1000
  const secondsInMinute = 60
  const minutesInHour = 60
  const hoursInDay = 24

  // const milliseconds = ms % millisecondsInSecond
  const totalSeconds = Math.floor(ms / millisecondsInSecond)
  const seconds = totalSeconds % secondsInMinute
  const totalMinutes = Math.floor(totalSeconds / secondsInMinute)
  const minutes = totalMinutes % minutesInHour
  const totalHours = Math.floor(totalMinutes / minutesInHour)
  const days = Math.floor(totalHours / hoursInDay)
  const hours = totalHours % hoursInDay

  return `${days}${t('timeFormat[0]')}${hours}${t('timeFormat[1]')}${minutes}${t('timeFormat[2]')}${seconds}${t('timeFormat[3]')}`
}
onBeforeUnmount(() => {
  clearInterval(timer)
})

</script>

<template>
  <div class="b-b b-b-dashed b-b-#eee p-4 flex justify-between items-center">
  <div class=" gap-2 flex">
    <b>{{ trans(route.params.key as string) }}</b>
    <q-badge><i class="i-iconamoon:type-bold mr-1"></i> {{ type?.toUpperCase() }}</q-badge>
    <q-badge><i class="i-material-symbols:memory-alt mr-1"></i> {{ dataToHuman(baseInfo.memory) }}</q-badge>
    <q-badge>
      <i class="i-material-symbols:nest-clock-farsight-analog-outline"></i>TTL
      {{ formatMilliseconds(baseInfo.pttl) }}
    </q-badge>
  </div>
    <slot></slot>
  </div>
</template>

<style scoped lang="scss"></style>
