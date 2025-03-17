<script setup lang="ts">
import { useReqStore } from "@/stores/req";
import { ElDropdown, ElDropdownMenu, ElDropdownItem, ElButton } from "element-plus"
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
  console.log(v)
  await router.replace({
    query: {
      ...route.query,
      db: v,
    }
  })
  location.reload()
}
console.log(route.query)
</script>
<template>
  <el-dropdown @command="reload">
    <span>
      <i class="i-material-symbols:database"></i> {{ $t('normal.0') + ' ' + db }}
    </span>
    <template #dropdown>
      <el-dropdown-menu>
        <el-dropdown-item :key="item" v-for="item in Number(dbs)" :command="item - 1">{{ $t('normal.0') + (item - 1)
        }}</el-dropdown-item>
      </el-dropdown-menu>
    </template>
  </el-dropdown>
</template>
<style scoped lang="scss"></style>
