import type { DialogApiInjection } from 'naive-ui/es/dialog/src/DialogProvider'
import NewStringForm from '@/pages/host/components/CoKeys/compoents/NewStringForm.vue'
import NewSetForm from '@/pages/host/components/CoKeys/compoents/NewSetForm.vue'
import NewListForm from '@/pages/host/components/CoKeys/compoents/NewListForm.vue'
import NewHashForm from '@/pages/host/components/CoKeys/compoents/NewHashForm.vue'

export const options = [
  {
    label: 'string',
    value: 'string',
  },
  {
    label: 'set',
    value: 'set',
  },
  {
    label: 'hash',
    value: 'hash',
  },
  {
    label: 'list',
    value: 'list',
  },
  {
    label: 'zset',
    value: 'zset',
  },
]

const template = {
  draggable: true,
  maskClosable: false,
}
export const addStringKey = (dialog: DialogApiInjection, title?: string) => {
  dialog.create({
    ...template,
    title: title ?? 'New Key',
    content: () => {
      return <NewStringForm></NewStringForm>
    },
  })
}

export const addListKey = (dialog: DialogApiInjection, title?: string) => {
  dialog.create({
    title: title ?? 'New Key',
    content: () => {
      return <NewListForm></NewListForm>
    },
    draggable: true,
  })
}

export const addHashKey = (dialog: DialogApiInjection, title?: string) => {
  dialog.create({
    title: title ?? 'New Key',
    content: () => {
      return <NewHashForm></NewHashForm>
    },
    draggable: true,
  })
}

export const addSetKey = (dialog: DialogApiInjection, title?: string) => {
  dialog.create({
    title: title ?? 'New Key',
    content: () => {
      return <NewSetForm></NewSetForm>
    },
    draggable: true,
  })
}
export const addZSetKey = (dialog: DialogApiInjection, title?: string) => {
  dialog.create({
    title: title ?? 'New Key',
    content: () => {
      return <div>good job</div>
    },
    draggable: true,
  })
}
