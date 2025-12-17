<script setup lang="tsx">
import { useReqStore } from '@/stores/req.ts'
import { ref, shallowRef, computed } from 'vue'
import { useRoute } from 'vue-router'
import { type ModalReactive } from 'naive-ui'
import { useI18n } from 'vue-i18n'
import CoInfoHeader from '@/pages/host/components/CoInfoHeader/index.vue'
import { Key } from '@/tools/keys.ts'
import { modal } from "@/tools"
// redis key
const key = Key()
// i18n helper function
const { t } = useI18n()
// route
const route = useRoute()
// hook, packaged request helper
const reqStore = useReqStore()
// redis key data
const content = shallowRef<string>('')
// redis key used space size. unit byte
const size = ref<string>('')
// key space size whether larger than limit size.
const largerThan = computed(() => {
  return Number(size.value) > 1024 * 1024
})
// fetch data
const fetchData = async () => {
  const { data } = await reqStore.reqWithHost<string>({
    path: '/cmd',
    data: ['get', atob(route.params.key as string)],
  })
  // more than 1Mb, we are truncate it.
  if (largerThan.value) {
    content.value = data.slice(0, 1024 * 1024) + '...'
    return
  }
  content.value = data
}
fetchData()
// change value model.
const vModel = ref<string>('')
// submit to change value
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
// create modal func.
const changeValue = () => {
  vModel.value = content.value
  const m = modal.create({
    title: t('actions.3'),
    preset: 'card',
    draggable: true,
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
  <div class="w-full h-full flex flex-col">
    <co-info-header v-model:size="size" type="string">
      <n-space>
        <n-button quaternary type="primary" size="small" @click="changeValue">
          <template #icon><i class="i-material-symbols:edit-square-rounded"></i></template> {{ $t('actions[3]') }}
        </n-button>
      </n-space>
    </co-info-header>
    <div class="p-4 flex-1 flex flex-col">
      <n-alert v-if="largerThan" type="info" class="mb-4" :bordered="false">
        {{ $t('tips[1]', { size: '1Mb' }) }}
        <n-button size="tiny" type="primary" quaternary>{{ $t("actions.6") }}</n-button>
      </n-alert>
      <n-input type="textarea" :resizable="false" readonly class="flex-1" v-model:value="content"></n-input>
    </div>
  </div>
</template>

<style scoped lang="scss"></style>
