<script setup lang="ts">
import { useReqStore } from '@/stores/req';
import { reactive, ref } from 'vue';
import { RecycleScroller } from 'vue-virtual-scroller'
import "vue-virtual-scroller/dist/vue-virtual-scroller.css"
import { ElTreeV2 } from 'element-plus'
import "element-plus/dist/index.css"
const reqStore = useReqStore()
// this is do search
const search = reactive({
  cursor: '0',
  match: '',
  keys: []
})

// this is no search
const keys = reactive({
  cursor: '0',
  keys: [] as Tree[],
})

const v = ref([])


// here we do some no search logic.
const noSearchKeys = async () => {
  const resp = await reqStore.reqWithHost<string>({
    path: '/cmd',
    db: '0',
    data: JSON.stringify(['scan', keys.cursor, 'MATCH', '*', 'COUNT', '5000'])
  })
  const v = resp.data.split('\n')
  let data = [] as Tree[]
  if (keys.cursor === '0') {
    data = v.splice(1).map(e => ({ type: 'key', label: e, icon: "key", id: ID() }))
  } else {
    data = keys.keys.concat(v.splice(1).map(e => ({ type: 'key', label: e, icon: "key", id: ID() })))
  }
  keys.cursor = v[0]
  const arr = [] as Tree[]
  data.forEach((e) => {
    parseLevel(arr, e.label)
  })
  console.log(arr)
  keys.keys = arr
}
noSearchKeys()

// tree object
interface Tree {
  label: string,
  icon: string,
  type: 'key' | 'folder',
  id: string,
  children?: Tree[]
}
const ID = () => Math.random().toString(36).slice(2)
// below code is aim to parse the key name to tree structure
const parseLevel = (ori: Tree[], key: string) => {
  // todo: split symbol should be configurable.
  // if not matched with `:` add it to tree structure immediately.
  if (!key.includes(':')) {
    ori.push({ label: key, icon: "key", type: 'key', id: ID() })
    return
  }
  // otherwise split it to and parse it to tree structure.
  const arr = key.split(':')
  // append children to root
  const appendChildren = (ori: Tree[], arr: string[]) => {
    // search root key.
    const root = ori.find((e) => e.label === arr[0] && e.type != 'key')
    // finded root key
    if (root) {
      // if not children, create it.
      if (!root.children) root.children = []
      // recursive parse it.
      appendChildren(root.children, arr.splice(1))
    } else {
      // if key length is less than 2, it means it is a key.
      if (arr.length <= 1) {
        const obj = { label: key, icon: "key", type: 'key', id: ID() } as Tree
        ori.push(obj)
        return
      }
      // otherwise create as a folder.
      const obj = { label: arr[0], icon: "folder", type: 'folder', children: [], id: ID() } as Tree
      // push it to as root.
      ori.push(obj)
      // and recursive parse it.
      appendChildren(obj.children!, arr.splice(1))
    }
  }
  // parse it
  appendChildren(ori, arr)
}
// todo: configurable name space enable.
const nameSpaceEnable = ref(true)
const icon = (e) => {
  console.log(e.target)
}
</script>
<template>
  <div class="flex flex-col h-full">
    <div class="p-2 flex-1 flex justify-center items-center flex-row">
      <input type="text" class="flex-1 outline-none" v-model="search.match">
    </div>
    <q-scroll-area v-if="nameSpaceEnable" class="h-[calc(100vh-120px)]">
      <!-- <q-tree dense :nodes="keys.keys" node-key="id" no-transition /> -->
      <el-tree-v2 style="height: calc(100vh - 120px)" :data="keys.keys" :props="{
        value: 'id',
        label: 'label',
        children: 'children',
      }" />
    </q-scroll-area>
    <RecycleScroller v-else class="h-[calc(100vh-120px)] scroller mx-2" :items="keys.keys" :item-size="32"
      key-field="label" v-slot="{ item }">
      <div class="flex items-center  mx-2 px-2 justify-start white-space-nowrap w-full cursor-pointer hover:bg-gray-100"
        style="flex-wrap: nowrap; word-break: keep-all; overflow: hidden; text-overflow: ellipsis;">
        <i class="i-meteor-icons:key mr-1" style="flex-shrink: 0;"></i>
        <span>{{ item.label }}</span>
      </div>
    </RecycleScroller>
    <div class="flex justify-center items-center flex-1">
      <span v-if="keys.cursor !== '0'" @click="noSearchKeys">加载更多</span>
    </div>
  </div>
</template>

<style lang="scss" scoped>
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
