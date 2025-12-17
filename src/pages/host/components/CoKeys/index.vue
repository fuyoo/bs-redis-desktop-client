<script setup lang="tsx">
import { useReqStore } from '@/stores/req.ts'
import { reactive, ref, h, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { type DropdownOption, type TreeOption } from 'naive-ui'
import { Folder, FolderOpenOutline, KeyOutline, Trash } from '@vicons/ionicons5'
import { NIcon } from 'naive-ui'
import { ID } from '@/tools/keys.ts'
import type { RedisKeyType, Tree } from '@/types.ts'
import KeysWorker from '@/worker/keys.ts?worker'
import { useI18n } from 'vue-i18n'
import { dialog } from '@/tools'
import {
  useActions,
} from '@/pages/host/components/CoKeys/actions.tsx'
const { addHashKey,
  addListKey,
  addSetKey,
  addStringKey,
  addZSetKey,
  options, } = useActions(dialog)
const keysWorker = new KeysWorker()
const router = useRouter()
const reqStore = useReqStore()
const route = useRoute()
const { t } = useI18n()
let originalKeyList = [] as Tree[]
// this is to do search
const search = reactive<{
  tree: Tree[]
  cursor: string
  loading: boolean
  match: string
}>({
  cursor: '0',
  match: '',
  tree: [],
  loading: false,
})
// here is not search object
const original = reactive<{
  tree: Tree[]
  cursor: string
  loading: boolean
}>({
  tree: [],
  cursor: '0',
  loading: false,
})

// todo: enable name space by configure.
const nameSpaceEnable = ref(true)
// here we do some no search patten logic.
const queryData = async (isSearch?: boolean) => {
  if (isSearch) search.loading = true
  else original.loading = true
  try {
    // fetch data pass through rust side.
    const resp = await reqStore.reqWithHost<string>({
      path: '/cmd',
      data: JSON.stringify([
        'scan',
        isSearch ? search.cursor : original.cursor,
        'MATCH',
        isSearch ? search.match : '*',
        'COUNT',
        '5000',
      ]),
    })
    // parse data
    const v = resp.data.split('\n')
    if (isSearch) {
      let arr = []

      if (search.cursor === '0') {
        arr = v.splice(1).map((e) => ({ type: 'key', label: e, icon: 'key', id: ID() }) as Tree)
      } else {
        arr = search.tree.concat(
          v.splice(1).map(
            (e) =>
              ({
                type: 'key',
                label: e,
                icon: 'key',
                id: ID(),
              }) as Tree,
          ),
        )
      }
      search.cursor = v[0]
      // const data = [] as Tree[]
      // arr.forEach((e) => {
      //   parseTreeWithNameSpace(data, e.label)
      // })
      // search.tree = data
      keysWorker.postMessage({
        type: 'parse',
        data: arr,
      })
      search.tree = await new Promise((resolve) => {
        keysWorker.onmessage = (e) => {
          resolve(e.data)
        }
      })
    } else {
      // no search match
      if (original.cursor === '0') {
        originalKeyList = v
          .splice(1)
          .map((e) => ({ type: 'key', label: e, icon: 'key', id: ID() }) as Tree)
      } else {
        v.splice(1).forEach((e) => {
          originalKeyList.push({ type: 'key', label: e, icon: 'key', id: ID() })
        })
      }
      original.cursor = v[0]
      keysWorker.postMessage({
        type: 'parse',
        data: originalKeyList,
      })
      original.tree = await new Promise((resolve) => {
        keysWorker.onmessage = (e) => {
          resolve(e.data)
        }
      })
    }
  } catch (e) {
    console.error(e)
  }
  original.loading = false
  search.loading = false
}
queryData()
const updatePrefixWithExpand = (
  _keys: Array<string | number>,
  _option: Array<TreeOption | null>,
  meta: {
    node: TreeOption | null
    action: 'expand' | 'collapse' | 'filter'
  },
) => {
  if (!meta.node) return
  switch (meta.action) {
    case 'expand':
      meta.node.prefix = () =>
        h(NIcon, null, {
          default: () => h(FolderOpenOutline),
        })
      break
    case 'collapse':
      meta.node.prefix = () =>
        h(NIcon, null, {
          default: () => h(Folder),
        })
      break
  }
}

const loadMoreFn = () => {
  if (search.match !== '') {
    queryData(true)
  } else {
    queryData()
  }
}

let focusNodeData: TreeOption | null
const showDropdownRef = ref(false)
const menuOptions = ref<DropdownOption[]>([
  {
    label: '删除',
    key: 'delete',
    icon: () => h(NIcon, null, { default: () => h(Trash) }),
  },
])
const x = ref(0)
const y = ref(0)
const supportDataType = ['string', 'list', 'set', 'zset', 'hash', 'none']
const nodeProps = ({ option }: { option: TreeOption }) => {
  return {
    async onClick() {
      if (option.type === 'key') {
        const t = await reqStore.reqWithHost<string>({
          path: '/cmd',
          data: ['type', option.value],
        })
        if (!supportDataType.includes(t.data)) {
          await router.replace({
            path: `/tab/${route.params.id}/main/database/unsupported/${btoa(option.value! as string)}`,
            replace: true,
            query: {
              ...route.query,
            },
          })
          return
        }
        await router.replace({
          path: `/tab/${route.params.id}/main/database/${t.data}/${btoa(option.value! as string)}`,
          replace: true,
          query: {
            ...route.query,
          },
        })
      }
    },
    onContextmenu(e: MouseEvent): void {
      showDropdownRef.value = true
      x.value = e.clientX
      y.value = e.clientY
      e.preventDefault()
      focusNodeData = option
    },
  }
}
// contextmenu selected function.
const handleSelect = async (act: string) => {
  showDropdownRef.value = false
  const data = focusNodeData
  if (act === 'delete') {
    if (data?.type === 'key') {
      const { code } = await reqStore.reqWithHost<string>({
        path: '/cmd',
        data: ['del', data.value],
      })
      if (code === 0) {
        await queryData(search.match !== '')
      }
      return
    }
    // delete all of children
    const keys = [] as string[]
    const findKeys = (data: Record<string, any>[]) => {
      for (const item of data || []) {
        if (item.type === 'key') {
          keys.push(item.value)
        } else {
          findKeys(item.children)
        }
      }
    }
    findKeys(data?.children || [])
    // todo: need implement batch delete at 'rust' end. this implementation is so ugly.
    for (const key of keys) {
      await reqStore.reqWithHost<string>({
        path: '/cmd',
        data: ['del', key],
      })
    }
    await queryData(search.match !== '')
  }
}
// throttle
let timer = -1 as any
const doFilter = async () => {
  search.cursor = '0'
  clearTimeout(timer)
  if (search.match === '') return
  timer = setTimeout(async () => {
    await queryData(true)
  }, 100)
}

function renderPrefix(data: { option: TreeOption }) {
  if (data.option.type === 'key') {
    return <NIcon> <KeyOutline /> </NIcon>
  } else {
    return <NIcon> <Folder /> </NIcon>
  }
}

// add
const addFn = (v: RedisKeyType) => {
  switch (v) {
    case 'string':
      addStringKey(t('title.0', { type: 'string' }))
      break
    case 'hash':
      addHashKey(t('title.0', { type: 'hash' }))
      break
    case 'set':
      addSetKey(t('title.0', { type: 'set' }))
      break
    case 'list':
      addListKey(t('title.0', { type: 'list' }))
      break
    case 'zset':
      addZSetKey(t('title.0', { type: 'zset' }))
      break
  }
}
const refreshFn = (s: boolean) => {
  if (s) {
    search.tree = []
  } else {
    original.tree = []
  }
  queryData(s)
}
</script>
<template>
  <div ref="treeBoxRef" class="w-full flex flex-col flex-1 justify-start items-start h-full">
    <div class=" shadow w-full">
      <div class="p-2 flex gap-2">
        <n-input size="small" clearable v-model:value="search.match" @update:value="doFilter"
          placeholder="redis query format">
          <template #prefix>
            <i class="i-material-symbols:database-search-rounded"></i>
          </template>
        </n-input>
        <n-popselect :on-update:value="addFn" :options="options" trigger="click">
          <n-button size="small" tertiary circle type="primary">
            <i class="i-material-symbols:add"></i>
          </n-button>
        </n-popselect>
      </div>
    </div>
    <div class="flex-1 relative w-full">
      <n-tree ref="treeInstRef" class="h-full " block-node show-line ellipsis :render-prefix="renderPrefix"
        :on-update:expanded-keys="updatePrefixWithExpand" :data="search.match !== '' ? search.tree : original.tree"
        virtual-scroll expand-on-click :node-props="nodeProps" key-field="id" children-field="children" />
    </div>
    <div class="flex w-full justify-center items-center py-2">
      <n-space v-if="!search.match">
        <n-button tertiary v-show="original.cursor !== '0'" size="small" :loading="original.loading"
          @click="loadMoreFn">
          <template #icon>
            <i class="i-material-symbols-light:text-select-move-forward-word-rounded rotate-90"></i>
          </template>
        </n-button>
        <n-button tertiary @click="refreshFn(false)" :loading="original.loading" v-show="original.cursor === '0'"
          size="small">
          <template #icon>
            <i class="i-material-symbols-light:directory-sync"></i>
          </template>
        </n-button>
      </n-space>
      <n-space v-else>
        <n-button tertiary v-show="search.cursor !== '1'" size="small" :loading="search.loading"
          @click="loadMoreFn"><template #icon>
            <i class="i-material-symbols-light:text-select-move-forward-word-rounded rotate-90"></i>
          </template>
        </n-button>
        <n-button tertiary :loading="search.loading" @click="refreshFn(true)" v-show="search.cursor === '0'"
          size="small">
          <template #icon>
            <i class="i-material-symbols-light:directory-sync"></i>
          </template>
        </n-button>
      </n-space>
    </div>
  </div>
  <n-dropdown trigger="manual" placement="bottom-start" :show="showDropdownRef" :options="menuOptions" :x="x" :y="y"
    @select="handleSelect" @clickoutside="() => (showDropdownRef = false)" />
</template>

<style lang="scss" scoped></style>
