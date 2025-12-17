<script setup lang="tsx">
import { reactive, ref, shallowRef } from 'vue'
import { useReqStore } from '@/stores/req.ts'
import CoInfoHeader from '@/pages/host/components/CoInfoHeader/index.vue'
import { ID, Key } from '@/tools/keys.ts'
import { usePager } from '@/hooks/pager.tsx'
import { useI18n } from 'vue-i18n'
import { useResize } from '@/hooks/life.ts'
import {message} from "@/tools/index.ts"
import type { FormInst } from 'naive-ui'
const dialog  = useDialog()
const { t } = useI18n()
const size = ref(0)
const key = Key()
const reqStore = useReqStore()
const { pager, onPageChanged, calcIndex } = usePager()

// fetch total count
const fetchLen = async () => {
  const resp = await reqStore.reqWithHost<string>({
    path: '/cmd',
    data: ['llen', key],
  })
  pager.itemCount = Number(resp.data)
  pager.pageCount = Math.ceil(Number(resp.data) / pager.pageSize)
  console.log(`pager.pageCount`, resp)
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
  { title: t('table[0]'), key: 'value', ellipsis: {
      tooltip: true
    }},
  {
    title: t('table[1]'),
    key: 'action',
    width: 200,
    render: (row: Record<string, any>) => {

      const delFn = async (row: Record<string, any>) => {
        try {
          row.delLoaing = true
          await reqStore.reqWithHost<string>({
            path: '/cmd',
            data: ['lrem', key, '1', row.value],
          })
          fetchRecords()
        } finally {
          row.delLoaing = false
        }
      }
      const look =  (row: Record<string, any>) => {
        dialog.create({
          title: t('actions[12]'),
          content:() => <n-input type="textarea" rows={8} class="my-4" readonly v-model:value={row.value} />,
        })
      }
      return (
        <n-space>
          {(pager.itemCount > 1) && <n-button type="warning" size="tiny" quaternary onClick={() => delFn(row)}>
            {t('actions[2]')}
          </n-button>}
          <n-button type="primary" size="tiny" quaternary onClick={() => editFn(row,calcIndex(row.no))}>
            {t('actions[3]')}
          </n-button>
          <n-button type="primary" size="tiny" onClick={()=>look(row)} quaternary>
            {t('actions[12]')}
          </n-button>
        </n-space>
      )
    },
  },
])
const records = shallowRef<Record<string, any>[]>([])
const fetchRecords = async () => {
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
const addData = async () => {
  const formModel = reactive({
    value: '',
    cmd: 'rpushx'
  })
  const rules = {
    value: [{
      required: false,
      message: t('keyForm.msg.1'),
      trigger: 'blur',
    }],
  }
  const formRef = ref<FormInst>()
  const submitFn = async () => {
    try {
      console.log('formModel', formModel)
      await formRef.value?.validate()
      await reqStore.reqWithHost<string>({
        path: '/cmd',
        data: [formModel.cmd, key, formModel.value],
      })
      message.destroyAll()
      message.success('ok')
      fetchRecords()
      dialog.destroyAll()
    } catch (e) {
      console.error(e)
    }
  }
  dialog.create({
    title: t('tips.5'),
    content:() => <n-form ref={formRef} mode={formModel} rules={rules} label-placement="left" label-width="80">
      <n-form-item label={t('keyForm.label.1')} path="value" >
        <n-input type="textarea" v-model:value={formModel.value}  />
      </n-form-item>
       <n-form-item label={t('keyForm.label.4')}>
         <n-select v-model:value={formModel.cmd} options={[{
    label: 'start',
    value: 'lpushx',
  },{
    label: 'end',
    value: 'rpushx',
  },]} />
      </n-form-item>
      <n-form-item label={' '}>
        <n-space>
          <n-button type="primary" onClick={() => submitFn()}>{t('actions[9]')}</n-button>
        </n-space>
      </n-form-item>
    </n-form>,
  })
}
const editFn = async (row:Record<string,any>,index: number) => {
  const formModel = reactive({
    value: row.value,
  })
  const rules = {
    value: [{
      required: false,
      message: t('keyForm.msg.1'),
      trigger: 'blur',
    }],
  }
  const formRef = ref<FormInst>()
  const submitFn = async () => {
    try {
      console.log('formModel', formModel)
      await formRef.value?.validate()
      await reqStore.reqWithHost<string>({
        path: '/cmd',
        data: ['lset', key, `${index-1}`, formModel.value],
      })
      message.destroyAll()
      message.success('ok')
      fetchRecords()
      dialog.destroyAll()
    } catch (e) {
      console.error(e)
    }
  }
  dialog.create({
    title: t('tips.6'),
    content:() => <n-form ref={formRef} mode={formModel} rules={rules} label-placement="left" label-width="80">
      <n-form-item label={t('keyForm.label.4')} path="value" >
        <n-input type="textarea" v-model:value={formModel.value}  />
      </n-form-item>
      <n-form-item label={' '}>
        <n-space>
          <n-button type="primary" onClick={() => submitFn()}>{t('actions[9]')}</n-button>
        </n-space>
      </n-form-item>
    </n-form>,
  })
}
</script>

<template>
  <div class="w-full h-full">
    <co-info-header v-model:size="size" type="string">
      <div><n-button @click="addData" quaternary size="small" type="primary">
          <template #icon><i class="i-pajamas:insert"></i></template>
          {{ $t('actions[5]') }}</n-button></div>
    </co-info-header>
    <div class="p-5">
      <n-data-table size="small" remote ref="table" :columns="columns" :data="records" :loading="reqStore.reqLoading"
        :pagination="pager" :style="{ height: `${height - 190}px` }" flex-height />
    </div>
  </div>
</template>

<style scoped lang="scss"></style>
