<script setup lang="tsx">
import { ref, shallowRef } from 'vue'
import { useRoute } from 'vue-router'
import { useReqStore } from '@/stores/req.ts'
import CoInfoHeader from '@/pages/host/components/CoInfoHeader/index.vue'
import { ID } from '@/tools/keys.ts'

const size = ref(0)
const route = useRoute()
const key = atob(route.params.key as string)
console.log(key)
const reqStore = useReqStore()
const total = ref(0)
const paginationReactive = reactive({
  page: 1,
  pageSize: 5,
  pageCount: 0,
  showSizePicker: true,
  pageSizes: [3, 5, 7],
  onChange: (page: number) => {
    paginationReactive.page = page
  },
  onUpdatePageSize: (pageSize: number) => {
    paginationReactive.pageSize = pageSize
    paginationReactive.page = 1
  }
})
const fetchLen = async () => {
  const resp = await reqStore.reqWithHost<string>({
    path: '/cmd',
    data: ['llen', key],
  })
  paginationReactive.pageCount = Number(resp.data)
}
fetchLen()


const columns = [
  {
    title: 'index',
    key: 'no',
    render: (row: Record) => {
      return <div> { (paginationReactive.page - 1) * paginationReactive.pageSize + row.no +1 } </div>
    },
  },
  { title: 'data', key: 'value' },
]
const records = shallowRef<Record[]>([])
const fetchRecords = async () => {
  await fetchLen()
  const resp = await reqStore.reqWithHost({
    path: '/cmd',
    data: ['lrange', key, `${(paginationReactive.page - 1) * paginationReactive.pageSize}`, `${paginationReactive.page * paginationReactive.pageSize}`],
  })
  console.log( ['lrange', key, `${(paginationReactive.page - 1) * paginationReactive.pageSize}`, `${paginationReactive.page * paginationReactive.pageSize}`],
  )
  console.log(resp)
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
</script>

<template>
  <div class="w-full h-full">
    <co-info-header v-model:size="size" type="string" />
    <n-data-table
      remote
      ref="table"
      :columns="columns"
      :data="records"
      :loading="reqStore.reqLoading"
      :pagination="paginationReactive"
    />
  </div>
</template>

<style scoped lang="scss"></style>
