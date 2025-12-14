<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { dialog } from '@/tools'
import { useI18n } from 'vue-i18n'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useRoute, useRouter } from 'vue-router'

const router = useRouter()
const route = useRoute()
const homePageId = 'home-page'
// focus tab.
const name = ref('')
if (route.path.indexOf('/home') > -1) name.value = 'home-page'
onMounted(() => {
  const d = document.querySelector('.n-tabs-pad')
  if (d) {
    d.innerHTML = `<div data-tauri-drag-region style="height:100%;width:100%;"></div>`
  }
})
const tabList = ref<Tab[]>([])
const getTabs = async () => {
  const resp = await invoke<BackendResponse<Tab[]>>('tab_list')
  tabList.value = [
    {
      id: 'home-page',
      name: t('home.name'),
    },
    ...resp.data,
  ]
}
getTabs()
enum Operation {
  mini,
  max,
  exit,
}
const { t } = useI18n()
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
const isHome = computed(() => {
  return name.value == homePageId
})
const handleSetting = () => {
  name.value = ''
  router.push('/settings')
}

const handleBeforeLeave = async (tabName: string) => {
  if (tabName == 'home-page') {
    await router.push({
      path: `/home`,
      query: route.query,
    })
    return true
  }
  return true
}
</script>

<template>
  <div class="flex flex-col w-100vw h-100vh">
    <div class="relative z-10" data-tauri-drag-region>
      <n-tabs
        tab-class="py-1!"
        class="h-40px"
        data-tauri-drag-region
        size="small"
        type="card"
        @before-leave="handleBeforeLeave"
        v-model:value="name"
      >
        <template #prefix>
          <div class="px-4 relative">
            bs
            <div class="absolute left-0 top-0 w-full h-full" data-tauri-drag-region></div>
          </div>
        </template>
        <n-tab
          class="mt-10px"
          v-for="item in tabList"
          :closable="item.id != homePageId"
          :key="item.id"
          :label="item.name"
          :name="item.id"
          ><template #default>
            <n-space :size="6">
              <i class="i-material-symbols:home-app-logo" v-if="isHome"></i>
              <i class="i-material-symbols:home-app-logo" v-else></i>
              {{ item.name }}
            </n-space>
          </template></n-tab
        >
        <template #suffix>
          <n-space class="pr-3">
            <n-button @click="handleSetting" quaternary circle size="small">
              <template #icon>
                <i class="i-ic:outline-settings"></i>
              </template>
            </n-button>
            <n-button @click="handleOperation(Operation.mini)" quaternary circle size="small">
              <template #icon>
                <i class="i-material-symbols:check-indeterminate-small"></i>
              </template>
            </n-button>
            <n-button @click="handleOperation(Operation.max)" quaternary circle size="small">
              <template #icon>
                <i class="i-fluent:full-screen-maximize-16-filled"></i>
              </template>
            </n-button>
            <n-button
              @click="handleOperation(Operation.exit)"
              type="error"
              quaternary
              circle
              size="small"
            >
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
