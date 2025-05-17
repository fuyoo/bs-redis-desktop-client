<script setup lang="ts">
import { useReqStore } from '@/stores/req.ts'
import { onBeforeUnmount, onMounted, reactive, ref, shallowRef } from 'vue'
import { useRoute, useRouter } from 'vue-router'

// tree object
interface Tree {
  label: string
  value?: string
  icon: string
  type: 'key' | 'folder'
  id: string
  children?: Tree[]
}

const router = useRouter()
const reqStore = useReqStore()
const route = useRoute()
// this is to do search
const search = reactive<{
  tree: Tree[]
  cursor: string
  loading: boolean,
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

const ID = () => Math.random().toString(36).slice(2)

const createTreeNode = (label: string, type: 'key' | 'folder', value?: string): Tree => {
  try {
    return { label, icon: type === 'key' ? 'key' : 'folder', type, id: ID(), value }
  } catch (error) {
    console.error('Error creating tree node:', error)
    throw error
  }
}
// below code is aim to parse the key name to tree structure
const parseLevel = (ori: Tree[], key: string, delimiter: string = ':') => {
  // Handle empty key
  if (!key) {
    return
  }

  // If not matched with delimiter, add it to tree structure immediately.
  if (!key.includes(delimiter)) {
    ori.push(createTreeNode(key, 'key', key))
    return
  }

  // Otherwise split it and parse it to tree structure.
  const arr = key.split(delimiter)

  const appendChildren = (ori: Tree[], arr: string[]) => {
    // Search root key.
    const root = ori.find((e) => e.label === arr[0] && e.type !== 'key')

    // Found root key
    if (root) {
      // If not children, create it.
      if (!root.children) root.children = []
      // Recursive parse it.
      appendChildren(root.children, arr.slice(1))
    } else {
      // If key length is less than 2, it means it is a key.
      if (arr.length <= 1) {
        ori.push(createTreeNode(arr[0], 'key', key))
        return
      }
      // Otherwise create as a folder.
      const obj = createTreeNode(arr[0], 'folder')
      obj.children = []
      // Push it to as root.
      ori.push(obj)
      // And recursive parse it.
      appendChildren(obj.children!, arr.slice(1))
    }
  }

  // Parse it
  try {
    appendChildren(ori, arr)
  } catch (error) {
    console.error('Error parsing level:', error)
  }
}
// here we do some no search patten logic.
const queryOriginalData = async () => {
  original.loading = true
  try {
    // fetch data pass through rust side.
    const resp = await reqStore.reqWithHost<string>({
      path: '/cmd',
      data: JSON.stringify(['scan', original.cursor, 'MATCH', '*', 'COUNT', '5000']),
    })
    // parse data
    const v = resp.data.split('\n')

    let data: Tree[] = []
    if (original.cursor === '0') {
      data = v.splice(1).map((e) => ({ type: 'key', label: e, icon: 'key', id: ID() }))
    } else {
      data = original.tree.concat(
        v.splice(1).map((e) => ({ type: 'key', label: e, icon: 'key', id: ID() })),
      )
    }
    original.cursor = v[0]
    const arr = [] as Tree[]
    data.forEach((e) => {
      parseLevel(arr, e.label)
    })
    original.tree = arr
    elTreeV2Ref.value?.setData(original.tree)
  } catch (e) {}
  original.loading = false
}
queryOriginalData()


// todo: enable name space by configure.
const nameSpaceEnable = ref(true)
const elTreeV2Ref = shallowRef()
const treeHeight = shallowRef(100)
let resizeTimer = 0 as any
const onResize = () => {
  clearTimeout(resizeTimer)
  resizeTimer = setTimeout(() => {
    const dom = document.querySelector('#app')
    treeHeight.value = (dom?.getBoundingClientRect()?.height || 0) - 160
    console.log(treeHeight.value)
  }, 100)
}
// dynamic set tree height
onMounted(() => {
  window.addEventListener('resize', onResize)
  onResize()
})

onBeforeUnmount(() => {
  window.removeEventListener('resize', onResize)
  clearTimeout(resizeTimer)
})

const onNodeClick = async (data: Record<string, any>, node: TreeNode, e: MouseEvent) => {
  if (data.type === 'key') {
    const t = await reqStore.reqWithHost<string>({
      path: '/cmd',
      data: ['type', data.value],
    })
    await router.replace({
      path: `/tab/${route.params.id}/main/database/${t.data}/${btoa(data.value)}`,
      replace: true,
      query: {
        ...route.query,
      },
    })
  }
}
const loadMoreFn = () => {
  if (search.match !== '') {
  } else {
    queryOriginalData()
  }
}
const delFn = async (data: Record<string, any>) => {
  if (data.type === 'key') {
    const { code } = await reqStore.reqWithHost<string>({
      path: '/cmd',
      data: ['del', data.value],
    })
    if (code === 0) {
      await queryOriginalData()
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
  findKeys(data.children)
  // todo: need implement batch delete at 'rust' end. this implementation so ugly.
  for (const key of keys) {
    await reqStore.reqWithHost<string>({
      path: '/cmd',
      data: ['del', key],
    })
  }
  await queryOriginalData()
}
const fillData = () => {
  for (let i = 0; i < 10000; i++) {
    reqStore.reqWithHost<string>({
      path: '/cmd',
      data: ['set', 'key-' + i, Math.random().toString(36).slice(2)],
    })
  }
}
</script>
<template>
  <div
    ref="treeBoxRef"
    class="overflow-hidden _mc w-full flex flex-col flex-1 justify-start items-start"
  >
     <div class="flex gap-1 p-2">
       <n-input clearable class="flex-1" v-model:value="search.match" placeholder="redis query format" >
         <template #prefix>
           <i class="i-material-symbols:database-search-rounded"></i>
         </template>
       </n-input>
       <n-button round  type="primary">
         <i class="i-material-symbols:add"></i>
       </n-button>
     </div>
    <div class="flex-1 w-full">
      <n-tree
        ref="treeInstRef"
        block-line
        :data="original.tree"
        default-expand-all
        virtual-scroll
        :style="{height: treeHeight+'px'}"
        key-field="id"
        children-field="children"
      />
    </div>
    <n-button type="primary" @click="fillData">填充数据</n-button>
    <div class="flex justify-center items-center">
      <n-button
        size="small"
        class="w-full h-full"
        v-show="original.cursor !== '0'"
        text
        :loading="reqStore.reqLoading"
        @click="loadMoreFn"
        >加载更多
      </n-button>
    </div>
  </div>
</template>

<style lang="scss" scoped>
._mc {
  &::v-deep(.el-tree-node) {
    padding-right: 10px;
    padding-left: 20px;

    * {
      line-height: 1;
    }

    .el-tree-node__expand-icon {
      background: url('@/assets/folder-close.png') no-repeat center center;
      background-size: 14px 14px;
      position: relative;

      &::after {
        content: '';
        position: absolute;
        top: -50%;
        left: -8px;
        height: 100%;
        width: 10px;
        border-left: 1px dashed #aaa;
        border-bottom: 1px dashed #aaa;
      }

      svg {
        display: none;
      }
    }

    .el-tree-node__expand-icon.expanded {
      background: url('@/assets/folder-open.png') no-repeat center center;
      background-size: 14px 14px;
      transform: rotate(0);

      &::after {
        top: -50%;
        height: 100%;
        width: 10px;
        border-left: 1px dashed #aaa;
        border-bottom: 1px dashed #aaa;
      }
    }

    .el-tree-node__expand-icon.is-leaf {
      background: url('@/assets/key.png') no-repeat center center;
      background-size: 14px 14px;
      visibility: visible;

      &::after {
        top: -50%;
        height: 100%;
        width: 10px;
        border-left: 1px dashed #aaa;
        border-bottom: 1px dashed #aaa;
      }
    }

    // width: calc(100% - 20px) !important;
    .el-tree-node__content {
      border-radius: 4px;
      font-size: 16px;
    }
  }
}

.scroller {
  &::-webkit-scrollbar {
    width: 8px;
    height: 8px;
    background-color: transparent;
  }

  &::-webkit-scrollbar-thumb {
    background: #ccc;
    border-radius: 4px;
  }
}
</style>
