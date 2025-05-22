<script setup lang="ts">
import { useReqStore } from '@/stores/req.ts'
import { reactive, ref, h,computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import type { DropdownOption } from 'naive-ui'
import { Folder, FolderOpenOutline, KeyOutline, Trash } from '@vicons/ionicons5'
import { NIcon } from 'naive-ui'
import { useResize } from '@/hooks/life.ts'
import { ID } from '@/tools/keys.ts'
import type { Tree } from '@/types.ts'
import KeysWorker from '@/worker/keys.ts?worker'

const keysWorker = new KeysWorker()
const router = useRouter()
const reqStore = useReqStore()
const route = useRoute()
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
      // const arr = [] as Tree[]
      // originalKeyList.forEach((e) => {
      //   parseTreeWithNameSpace(arr, e.label)
      // })
      // original.tree = arr
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

const { height } = useResize(115)
const calcHeight = computed(() => {
  if (search.match !== '') {
    if (search.cursor === '0') {
      return height.value + 30 + 'px'
    }
    return height.value + 'px'
  }
  if (original.cursor === '0') {
    return height.value + 30 + 'px'
  }
  return height.value + 'px'
})
const updatePrefixWithExpand = (
  _keys: Array<string | number>,
  _option: Array<Tree | null>,
  meta: {
    node: Tree | null
    action: 'expand' | 'collapse' | 'filter'
  },
) => {
  console.log ('updatePrefixWithExpand', _keys, _option, meta)
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

let focusNodeData: Tree | null
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
const supportDataType = ['string', 'list', 'set', 'zset', 'hash']
const nodeProps = ({ option }: { option: Tree }) => {
  return {
    async onClick() {
      if (option.type === 'key') {
        const t = await reqStore.reqWithHost<string>({
          path: '/cmd',
          data: ['type', option.value],
        })
        if (!supportDataType.includes(t.data)) {
          await router.replace({
            path: `/tab/${route.params.id}/main/database/unsupported/${btoa(option.value!)}`,
            replace: true,
            query: {
              ...route.query,
            },
          })
          return
        }
        await router.replace({
          path: `/tab/${route.params.id}/main/database/${t.data}/${btoa(option.value!)}`,
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
      console.log(e.clientX, e.clientY)
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
        await queryData(search.match !== "")
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
    await queryData(search.match !== "")
  }
}
// throttle
let timer = -1 as any
const doFilter = async () => {
  console.log(search)
  search.cursor = '0'
  clearTimeout(timer)
  if (search.match === '') return
  timer = setTimeout(async () => {
    await queryData(true)
  }, 100)
}

function renderPrefix(data: { option: Tree }) {
  return h(NIcon, null, {
    default: () => h(data.option.type === 'key' ? KeyOutline : Folder),
  })
}
</script>
<template>
  <div ref="treeBoxRef" class="w-full flex flex-col flex-1 justify-start items-start">
    <div class="flex gap-1 p-2 shadow w-full">
      <n-input
        clearable
        class="flex-1"
        size="tiny"
        v-model:value="search.match"
        @update:value="doFilter"
        placeholder="redis query format"
      >
        <template #prefix>
          <i class="i-material-symbols:database-search-rounded"></i>
        </template>
      </n-input>
      <n-button round type="primary" size="tiny">
        <i class="i-material-symbols:add"></i>
      </n-button>
    </div>
    <n-tree
      ref="treeInstRef"
      block-line
      show-line
      :render-prefix="renderPrefix"
      :on-update:expanded-keys="updatePrefixWithExpand"
      :data="search.match != '' ? search.tree : original.tree"
      virtual-scroll
      expand-on-click
      :node-props="nodeProps"
      :style="{ height: calcHeight }"
      key-field="id"
      children-field="children"
      class="whitespace-nowrap"
    />
    <div
      class="flex flex-1 w-full justify-center items-center"
      v-if="!search.match"
      v-show="original.cursor != '0'"
    >
      <n-button size="small" type="primary" :loading="reqStore.reqLoading" @click="loadMoreFn"
        >加载更多
      </n-button>
    </div>
    <div
      v-else
      class="flex flex-1 w-full justify-center items-center"
      v-show="search.cursor != '0'"
    >
      <n-button size="small" type="primary" :loading="reqStore.reqLoading" @click="loadMoreFn"
        >加载更多
      </n-button>
    </div>
  </div>
  <n-dropdown
    trigger="manual"
    placement="bottom-start"
    :show="showDropdownRef"
    :options="menuOptions as any"
    :x="x"
    :y="y"
    @select="handleSelect"
    @clickoutside="() => (showDropdownRef = false)"
  />
</template>

<style lang="scss" scoped></style>
