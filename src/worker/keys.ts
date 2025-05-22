import { parseTreeWithNameSpace } from '@/tools/keys.ts'
import {type Tree} from "@/types.ts"

self.addEventListener('message', (e) => {
  let data = [] as Tree[]
  e.data.data.forEach((e:Tree) => {
    parseTreeWithNameSpace(data, e.label)
  })
  self.postMessage(data)
  console.log(data)
});
