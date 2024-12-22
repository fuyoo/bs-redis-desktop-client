<template>
  <div class="flex-shrink-0">
    <q-btn
      flat
      round
      dense
      icon="remove"
      class="q-mr-xs"
      @click="handleOperation(Operation.mini)"
    />
    <q-btn
      flat
      round
      dense
      icon="fullscreen"
      class="q-mr-xs"
      @click="handleOperation(Operation.max)"
    />
    <q-btn flat round dense icon="close" @click="handleOperation(Operation.exit)" />
  </div>
</template>

<script setup lang="ts">
import { useQuasar } from 'quasar'
import { useI18n } from 'vue-i18n'
import { getAllWindows, getCurrentWindow } from '@tauri-apps/api/window'
import { useTabStore } from '../stores/tab'
enum Operation {
  reset,
  mini,
  max,
  exit,
}
const { t } = useI18n()
const $q = useQuasar()
const tabStore = useTabStore()
const handleOperation = async (o: Operation) => {
  switch (o) {
    case Operation.exit: {
      const tab = tabStore.focusTab()
      await tabStore.change({ id: 'main' })
      $q.dialog({
        transitionShow: 'rotate',
        title: '提示',
        message: t('exit'),
        ok: {
          push: true,
        },
        cancel: {
          push: true,
          color: 'negative',
        },
        persistent: true,
      })
        .onOk(async () => (await getAllWindows()).forEach((element) => element.close()))
        .onCancel(async () => {
          if (tab) await tabStore.change(tab)
        })
      break
    }
    case Operation.max:
      getCurrentWindow().toggleMaximize()
      break
    case Operation.mini:
      getCurrentWindow().minimize()
      break
  }
}
</script>
