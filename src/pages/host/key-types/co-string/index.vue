<script setup lang="tsx">
import { useReqStore } from '@/stores/req.ts'
import { ref, shallowRef } from 'vue'
import { useRoute } from 'vue-router'
import { useModal,type ModalReactive } from 'naive-ui'
import { useI18n } from 'vue-i18n'
import CoInfoHeader from '@/pages/host/components/CoInfoHeader/index.vue'
import { Key } from '@/tools/keys.ts'

const key = Key()
const { t } = useI18n()
const route = useRoute()
const reqStore = useReqStore()
const content = shallowRef<string>('')
const size = ref<string>('')
const fetchData = async () => {
  const { data } = await reqStore.reqWithHost<string>({
    path: '/cmd',
    data: ['get', atob(route.params.key as string)],
  })
  // more than 1Mb, we are truncate it.
  if (Number(size.value) > 1024 * 1024) {
    content.value = data.slice(0, 1024 * 1024) + '...'
    return
  }
  content.value = data
}
fetchData()
const vModel = ref<string>('')
const modal = useModal()
const submitFn = (m: ModalReactive) => {
  reqStore
    .reqWithHost({
      path: '/cmd',
      data: ['set', key, vModel.value],
    })
    .then(() => {
      m.destroy()
      fetchData()
    })
}
const changeValue = () => {
  vModel.value = content.value
  const m = modal.create({
    title: t('actions.3'),
    preset: 'card',
    draggable:true,
    maskClosable: false,
    style: {
      width: '580px',
    },
    content: () => {
      return (
        <>
          <n-input type="textarea" vModel:value={vModel.value} />
        </>
      )
    },
    footer: () => {
      return (
        <>
          <n-space>
            <n-button type="primary" onClick={() => submitFn(m)}>
              {t('actions[0]')}
            </n-button>
            <n-button type="primary" onClick={m.destroy}>
              {t('actions[1]')}
            </n-button>
          </n-space>
        </>
      )
    },
  })
}
</script>

<template>
  <div class="w-full h-full">
    <co-info-header v-model:size="size" type="string">
      <n-space>
        <n-button type="primary" size="tiny" @click="changeValue">
          <i class="i-material-symbols:edit-square-rounded"></i> {{ $t('actions[3]') }}
        </n-button>
      </n-space>
    </co-info-header>
    <div class="p-4">
      <textarea readonly class="w-full h-full resize-none b-none outline-none" v-model="content" />
    </div>
  </div>
</template>

<style scoped lang="scss"></style>
