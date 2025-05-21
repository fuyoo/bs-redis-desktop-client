import { h } from 'vue'
import { NIcon } from 'naive-ui'
import { Folder, KeyOutline } from '@vicons/ionicons5'
import type { Tree } from '@/types.ts'
import { useRoute } from 'vue-router'

export const ID = () => Math.random().toString(36).slice(2)

const createTreeNode = (label: string, type: 'key' | 'folder', value?: string): Tree => {
  try {
    return {
      label,
      icon: type === 'key' ? 'key' : 'folder',
      type,
      id: ID(),
      value,
      prefix: () =>
        h(NIcon, null, {
          default: () => h(type === 'key' ? KeyOutline : Folder),
        }),
    }
  } catch (error) {
    console.error('Error creating tree node:', error)
    throw error
  }
}
// below code is aim to parse the key name to tree structure
export const parseTreeWithNameSpace = (ori: Tree[], key: string, delimiter: string = ':') => {
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

// get redis key from route
export const Key = () => {
  const route = useRoute()
  return atob(route.params.key as string)
}
