<script setup lang="tsx">
import { reactive, ref, shallowRef } from 'vue'
import { useReqStore } from '@/stores/req.ts'
import CoInfoHeader from '@/pages/host/components/CoInfoHeader/index.vue'
import { ID, Key } from '@/tools/keys.ts'
import { useI18n } from 'vue-i18n'
import { useResize } from '@/hooks/life.ts'
import {message} from "@/tools/index.ts"
import type { FormInst } from 'naive-ui'
import { useActions } from '@/hooks/actions'
const dialog  = useDialog()
const {checkKeyIsExist} = useActions(dialog)
const { t } = useI18n()
const size = ref(0)
const key = Key()
const pageSize = ref('50')
const reqStore = useReqStore()
const sizes = [{label: '50',value: '50'},
{label: '100',value: '100'},
{label: '300',value: '300'},
{label: '500',value: '500'},
{label: '1000',value: '1000'}]
const pattern = ref('*')
// table fields
const columns = reactive([
  { title: '#', key: 'no',
    width: 80, ellipsis: {
      tooltip: true
    }},
  { title: t('table[0]'), key: 'value', ellipsis: {
      tooltip: true
    }},
  {
    title: t('table[1]'),
    key: 'action',
    width: 140,
    render: (row: Record<string, any>) => {
      // delete data
      const delFn = async (row: Record<string, any>) => {
        try {
          row.delLoaing = true
          await reqStore.reqWithHost<string>({
            path: '/cmd',
            data: ['srem', key, row.value],
          })
          try {
           await checkKeyIsExist(key, true, true)
          } catch (error) {
            console.error('Error:', error)
          }
          fetchRecords()
        } finally {
          row.delLoaing = false
        }
      }
      // look detail
      const look =  (row: Record<string, any>) => {
        dialog.create({
          title: t('actions[12]'),
          content:() => <n-input type="textarea" resizable={false} rows={8} class="my-4" readonly v-model:value={row.value} />,
        })
      }
      return (
        <n-space>
         {
         <n-button type="warning" size="tiny" quaternary onClick={() => delFn(row)}>
            {t('actions[2]')}
          </n-button>}
          <n-button type="primary" size="tiny" onClick={()=>look(row)} quaternary>
            {t('actions[12]')}
          </n-button>
        </n-space>
      )
    },
  },
])
// sscan command cursor
const cursor = ref('0')
//  records
const records = shallowRef<Record<string, any>[]>([])

// obtain records
const fetchRecords = async () => {
  // SSCAN key cursor [MATCH pattern] [COUNT count]
  const resp = await reqStore.reqWithHost<string>({
    path: '/cmd',
    data: [
      'sscan',
      key,
      cursor.value,
      'match',
      pattern.value,
      'count',
      pageSize.value,
    ],
  })

  const values  = resp.data.split('\n')
  cursor.value = values[0]
  records.value = values.slice(1).filter(i => i != '').map((item: string, i: number) => {
    return {
      title: item,
      value: item,
      no: i+1,
      key: ID(),
    }
  })
  console.log('records',values[0], values.slice(1))
}
fetchRecords()

const { height } = useResize()
// insert data to list
const addData = async () => {
  const formModel = reactive({
    value: '',
  })
  const rules = {
    value: [{
      required: true,
      message: t('keyForm.msg.1'),
      trigger: 'blur',
    }],
  }
  const formRef = ref<FormInst>()
  const submitFn = async () => {
    try {
      await formRef.value?.validate()
      await reqStore.reqWithHost<string>({
        path: '/cmd',
        data: ['sadd', key, formModel.value],
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
    content:() => <n-form ref={formRef} model={formModel} rules={rules} label-placement="left" label-width="80">
      <n-form-item label={t('keyForm.label.1')} path="value" required>
        <n-input v-model:value={formModel.value}  />
      </n-form-item>
      <n-form-item label={' '}>
        <n-space>
          <n-button type="primary" onClick={() => submitFn()}>{t('actions[9]')}</n-button>
        </n-space>
      </n-form-item>
    </n-form>,
  })
}
const resetFn = () => {
  cursor.value = '0'
  pattern.value = '*'
  fetchRecords()
}
const searchFn = () => {
  cursor.value = '0'
  fetchRecords()
}
</script>

<template>
  <div class="w-full h-full">
    <co-info-header v-model:size="size" type="set">
      <div><n-button @click="addData" quaternary size="small" type="primary">
          <template #icon><i class="i-pajamas:insert"></i></template>
          {{ $t('actions[5]') }}</n-button></div>
    </co-info-header>
    <div class="p-5">
      <div class="pb-5">
        <n-space>
          <n-select size="small" style="width: 80px;" v-model:value="pageSize" :options="sizes"></n-select>
          <n-input size="small" v-model:value="pattern" />
          <n-button @click="searchFn" size="small" type="primary">{{ $t('actions[13]') }}</n-button>
          <n-button size="small" ghost type="primary" @click="resetFn">{{ $t('actions[14]') }}</n-button>
          <n-button @click="fetchRecords" v-if="cursor != '0'" size="small" tertiary type="primary">{{ $t('actions[10]')
            }}</n-button>
        </n-space>
      </div>
      <n-data-table size="small" remote ref="table" :columns="columns" :data="records" :loading="reqStore.reqLoading"
        :pagination="false" :style="{ height: `${height - 240}px` }" flex-height />
    </div>
  </div>

</template>

<style scoped lang="scss"></style>
