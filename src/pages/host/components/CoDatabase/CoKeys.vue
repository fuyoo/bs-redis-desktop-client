<script setup lang="ts">
import { useReqStore } from '@/stores/req';
import { defineComponent, h, onMounted, reactive, ref } from 'vue';
import { RecycleScroller } from 'vue-virtual-scroller'
import "vue-virtual-scroller/dist/vue-virtual-scroller.css"
import { ElTreeV2, type TreeNode } from 'element-plus'
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
  // fetch data pass through rust side.
  const resp = await reqStore.reqWithHost<string>({
    path: '/cmd',
    data: JSON.stringify(['scan', keys.cursor, 'MATCH', '*', 'COUNT', '5000'])
  })
  // parse data
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
  keys.keys = arr
}
noSearchKeys()

// tree object
interface Tree {
  label: string,
  value?: string,
  icon: string,
  type: 'key' | 'folder',
  id: string,
  children?: Tree[]
}
const ID = () => Math.random().toString(36).slice(2)
// below code is aim to parse the key name to tree structure
const parseLevel = (ori: Tree[], key: string, delimiter: string = ':') => {
  // Handle empty key
  if (!key) {
    return;
  }

  // If not matched with delimiter, add it to tree structure immediately.
  if (!key.includes(delimiter)) {
    ori.push(createTreeNode(key, 'key'));
    return;
  }

  // Otherwise split it and parse it to tree structure.
  const arr = key.split(delimiter);

  const appendChildren = (ori: Tree[], arr: string[]) => {
    // Search root key.
    const root = ori.find((e) => e.label === arr[0] && e.type !== 'key');

    // Found root key
    if (root) {
      // If not children, create it.
      if (!root.children) root.children = [];
      // Recursive parse it.
      appendChildren(root.children, arr.slice(1));
    } else {
      // If key length is less than 2, it means it is a key.
      if (arr.length <= 1) {
        ori.push(createTreeNode(arr[0], 'key', key));
        return;
      }
      // Otherwise create as a folder.
      const obj = createTreeNode(arr[0], 'folder');
      obj.children = [];
      // Push it to as root.
      ori.push(obj);
      // And recursive parse it.
      appendChildren(obj.children!, arr.slice(1));
    }
  };

  // Parse it
  try {
    appendChildren(ori, arr);
  } catch (error) {
    console.error('Error parsing level:', error);
  }
};

const createTreeNode = (label: string, type: 'key' | 'folder', value?: string): Tree => {
  try {
    return { label, icon: type === 'key' ? "key" : "folder", type, id: ID(), value };
  } catch (error) {
    console.error('Error creating tree node:', error);
    throw error;
  }
};
// todo: configurable name space enable.
const nameSpaceEnable = ref(true)
const elTreeV2Ref = ref()
const treeHeight = ref(0)
// dynamic set tree height
onMounted(() => {
  window.addEventListener('resize', () => {
    treeHeight.value = (elTreeV2Ref.value.$el.getBoundingClientRect()).height
  })
  treeHeight.value = (elTreeV2Ref.value.$el.getBoundingClientRect()).height
})
const print = (data: TreeNode) => {
  console.log(data)
}
</script>
<template>
  <div class="flex flex-col h-full _mc">
    <div class="p-2 flex-1 flex justify-center items-center flex-row">
      <input type="text" class="flex-1 outline-none" v-model="search.match">
    </div>
    <el-tree-v2 :item-size="30" :height="treeHeight" v-if="nameSpaceEnable" class="h-[calc(100vh-160px)]"
      ref="elTreeV2Ref" :data="keys.keys" :props="{
        value: 'id',
        label: 'label',
        children: 'children',
      }">
      <template #default="{ node, data }">
        <div class="w-full text-nowrap  text-ellipsis cursor-default overflow-hidden" :title="data.value">
          <span>
            {{ data.label }}{{ print(node) }}</span>
        </div>
      </template>
    </el-tree-v2>
    <RecycleScroller v-else class="h-[calc(100vh-160px)] scroller mx-2" :items="keys.keys" :item-size="32"
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
