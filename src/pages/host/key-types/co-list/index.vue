<script setup lang="tsx">
import { reactive, ref, shallowRef } from 'vue'
import { useRoute } from 'vue-router'
import { useReqStore } from '@/stores/req.ts'
import CoInfoHeader from '@/pages/host/components/CoInfoHeader/index.vue'
import { ID, Key } from '@/tools/keys.ts'
import { usePager } from '@/hooks/pager.tsx'
import { useI18n } from 'vue-i18n'
import { useResize } from '@/hooks/life.ts'

const { t } = useI18n()
const size = ref(0)
const route = useRoute()
const key = Key()
const reqStore = useReqStore()
const { pager, onPageChanged, calcIndex } = usePager()
const fetchLen = async () => {
  const resp = await reqStore.reqWithHost<string>({
    path: '/cmd',
    data: ['llen', key],
  })
  pager.pageCount = Math.ceil(Number(resp.data) / pager.pageSize)
}
fetchLen()

const columns = reactive([
  {
    title: '#',
    key: 'no',
    render: (row: Record<string, any>) => {
      return <div> {calcIndex(row.no)} </div>
    },
  },
  { title: t('table[0]'), key: 'value' },
  {
    title: t('table[1]'),
    width: 200,
    render: (_: Record<string, any>) => {
      return (
        <n-space>
          <n-button type="warning" size="tiny" quaternary>
            {t('actions[2]')}
          </n-button>
          <n-button type="primary" size="tiny" quaternary>
            {t('actions[3]')}
          </n-button>
        </n-space>
      )
    },
  },
])
const records = shallowRef<Record<string, any>[]>([])
const fetchRecords = async () => {
  console.log('......')
  await fetchLen()
  const resp = await reqStore.reqWithHost<string>({
    path: '/cmd',
    data: [
      'lrange',
      key,
      `${(pager.page - 1) * pager.pageSize}`,
      `${pager.page * pager.pageSize - 1}`,
    ],
  })
  records.value = resp.data.split('\n').map((item: string, i: number) => {
    return {
      title: item,
      value: item,
      no: i,
      key: ID(),
    }
  })
}
fetchRecords()
onPageChanged(fetchRecords)
const { height } = useResize()
</script>

<template>
  <div class="w-full h-full">
    <co-info-header v-model:size="size" type="string">
      <div><n-button size="tiny" type="primary">{{ $t('actions[5]') }}</n-button></div>
    </co-info-header>
    <div class="p-5">
      <n-data-table
        remote
        ref="table"
        :columns="columns"
        :data="records"
        :loading="reqStore.reqLoading"
        :pagination="pager"
        :style="{ height: `${height - 200}px` }"
        flex-height
      />
    </div>
  </div>
</template>

<style scoped lang="scss"></style>
