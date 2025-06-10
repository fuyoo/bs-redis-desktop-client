import type { DialogApiInjection } from 'naive-ui/es/dialog/src/DialogProvider'
import NewStringForm from '@/pages/host/components/CoKeys/compoents/NewStringForm.vue'

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
      return <div>good job</div>
    },
    draggable: true,
  })
}

export const addHashKey = (dialog: DialogApiInjection, title?: string) => {
  dialog.create({
    title: title ?? 'New Key',
    content: () => {
      return <div>good job</div>
    },
    draggable: true,
  })
}

export const addSetKey = (dialog: DialogApiInjection, title?: string) => {
  dialog.create({
    title: title ?? 'New Key',
    content: () => {
      return <div>good job</div>
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
