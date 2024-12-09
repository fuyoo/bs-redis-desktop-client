<template>
  <div>
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
import { getAllWindows, getCurrentWindow } from '@tauri-apps/api/window'
enum Operation {
  reset,
  mini,
  max,
  exit,
}
const $q = useQuasar()
const handleOperation = async (o: Operation) => {
  switch (o) {
    case Operation.exit:
      $q.dialog({
        transitionShow: 'rotate',
        title: '提示',
        message: '你确定要关闭应用吗？',
        ok: {
          push: true,
          label: '确定',
        },
        cancel: {
          push: true,
          label: '取消',
          color: 'negative',
        },
        persistent: true,
      }).onOk(async () => (await getAllWindows()).forEach((element) => element.close()))
      break
    case Operation.max:
      getCurrentWindow().toggleMaximize()
      break
    case Operation.mini:
      getCurrentWindow().minimize()
      break
  }
}
</script>
