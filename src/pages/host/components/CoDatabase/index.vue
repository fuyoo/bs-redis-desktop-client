<script setup lang="ts">
import { useReqStore } from "@/stores/req.ts";
import { ElDropdown, ElDropdownMenu, ElDropdownItem } from "element-plus"
import { ref } from "vue";
import { useRoute, useRouter } from "vue-router";
const reqStore = useReqStore()
const router = useRouter()
const route = useRoute()
const dbs = ref("16")
const db = ref(route.query.db as string || "0")
const fetchDbs = async () => {
  const resp = await reqStore.reqWithHost<string>({
    path: '/cmd',
    data: ['config', 'get', 'databases']
  })
  dbs.value = resp.data.split("\n")[1];
}
fetchDbs()
const reload = async (v: number) => {
  if (v == Number(route.query.db || 0)) {
    return
  }
  await router.replace({
    query: {
      ...route.query,
      db: v,
    }
  })
  location.reload()
}
</script>
<template>
  <el-dropdown :max-height="248" @command="reload" trigger="click" class="w-full">
    <q-btn outline unelevated dense no-caps push long color="primary" class="w-full mx-2">
      <div style="width: 100%;">
        <i class="i-material-symbols:database"></i>
        {{ $t('normal.0') + '.' + db }}
      </div>
    </q-btn>
    <template #dropdown>
      <el-dropdown-menu>
        <el-dropdown-item :disabled="item - 1 == (route.query.db || 0)" :key="item" v-for="item in Number(dbs)"
          :command="item - 1">
          <div class="w-40 text-center">
            {{ $t('normal.0') +
              '.' +
              (item - 1) }}
          </div>
        </el-dropdown-item>
      </el-dropdown-menu>
    </template>
  </el-dropdown>
</template>
<style scoped lang="scss"></style>
