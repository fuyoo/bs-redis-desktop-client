<script setup lang="ts">
import { dialog } from '@/tools'
import { useI18n } from 'vue-i18n'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useRoute } from 'vue-router'
import { Platform } from "@/tools"
import { listen } from '@tauri-apps/api/event'
import { useTab } from '../hooks/tab'
import { useThemeVars } from 'naive-ui'
const vars = useThemeVars()
const bgColor = computed(() => {
  if (vars.value.baseColor === '#000') {
    return `rgb(34 34 34 )`
  } else {
    return `rgb(255 255 255)`
  }
})
console.log('baseColor', vars.value)
const route = useRoute()
const homePageId = 'main'
const settingsId = 'settings'
const { t } = useI18n()
// focus tab.
const name = ref('')
if (route.path.indexOf('/home') > -1) name.value = homePageId
onMounted(() => {
  const d = document.querySelector('.n-tabs-pad')
  if (d) {
    d.innerHTML = `<div data-tauri-drag-region style="height:100%;width:100%;"></div>`
  }
})
const { tabList, update, current, change, close } = useTab(t)
update()
  .then(() => {
    const tab = current()
    if (tab) {
      name.value = tab.id
    }
  })
const handleClose = async (tabName: string) => {
  console.log('close', tabName)
  close(tabName)
}
enum Operation {
  mini,
  max,
  exit,
}
const handleOperation = async (o: Operation) => {
  switch (o) {
    case Operation.exit: {
      dialog.create({
        content: t('exit'),
        negativeText: t('actions.1'),
        positiveText: t('actions.0'),
      })
      break
    }
    case Operation.max:
      await getCurrentWindow().toggleMaximize()
      break
    case Operation.mini:
      await getCurrentWindow().minimize()
      break
  }
}

const handleSetting = () => {
  change({
    id: settingsId,
    name: 'settings',
    url: "/#/settings"
  })
}

const handleBeforeLeave = async (tabName: string) => {
  const tab = tabList.value.find((e) => e.id == tabName)!
  change(tab)
  return false
}
const unlisten = listen("update-tabs", (evt) => {
  update()
})

onBeforeUnmount(async () => {
  (await unlisten)()
})
</script>

<template>
  <div class="flex flex-col w-100vw h-100vh bg-dark" :style="{
    background: bgColor
  }">
    <div class="relative z-10" data-tauri-drag-region>
      <n-tabs @close="handleClose" tab-class="py-1!" class="h-40px" data-tauri-drag-region size="small" type="card"
        @before-leave="handleBeforeLeave" v-model:value="name">
        <template #prefix>
          <div class="px-4 relative" :class="{ 'pl-50px': Platform.macos }">
            <div class="absolute left-0 top-0 w-full h-full" data-tauri-drag-region></div>
          </div>
        </template>
        <n-tab class="mt-10px" v-for="item in tabList" :closable="item.id != homePageId" :key="item.id"
          :label="item.name" :name="item.id"><template #default>
            <n-space :size="6">
              <i class="i-material-symbols:home-app-logo" v-if="item.id === homePageId"></i>
              <i class="i-material-symbols:database" v-else></i>
              {{ item.name }}
            </n-space>
          </template></n-tab>
        <template #suffix>
          <n-space class="pr-3">
            <n-button @click="handleSetting" quaternary circle size="small">
              <template #icon>
                <i class="i-ic:outline-settings"></i>
              </template>
            </n-button>
            <n-button v-if="!Platform.macos" @click="handleOperation(Operation.mini)" quaternary circle size="small">
              <template #icon>
                <i class="i-material-symbols:check-indeterminate-small"></i>
              </template>
            </n-button>
            <n-button v-if="!Platform.macos" @click="handleOperation(Operation.max)" quaternary circle size="small">
              <template #icon>
                <i class="i-fluent:full-screen-maximize-16-filled"></i>
              </template>
            </n-button>
            <n-button v-if="!Platform.macos" @click="handleOperation(Operation.exit)" type="error" quaternary circle
              size="small">
              <template #icon>
                <i class="i-material-symbols:close-rounded"></i>
              </template>
            </n-button>
          </n-space>
        </template>
      </n-tabs>
      <div class="w-full h-10px absolute left-0 top-0" data-tauri-drag-region></div>
    </div>
    <div class="flex-1 relative">
      <n-scrollbar class="absolute left-0 top-0 w-full h-full">
        <router-view />
      </n-scrollbar>
    </div>
  </div>
</template>

<style scoped lang="scss"></style>
