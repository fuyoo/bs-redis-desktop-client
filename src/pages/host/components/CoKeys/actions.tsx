import NewStringForm from '@/pages/host/components/CoKeys/compoents/NewStringForm.vue'
import NewSetForm from '@/pages/host/components/CoKeys/compoents/NewSetForm.vue'
import NewListForm from '@/pages/host/components/CoKeys/compoents/NewListForm.vue'
import NewHashForm from '@/pages/host/components/CoKeys/compoents/NewHashForm.vue'
import type { DialogApiInjection } from 'naive-ui/es/dialog/src/DialogProvider'

export const useActions = (dialog: DialogApiInjection) => {
  const options = [
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
  const addStringKey = (title?: string) => {
    dialog.create({
      ...template,
      title: title ?? 'New Key',
      content: () => {
        return <NewStringForm></NewStringForm>
      },
    })
  }

  const addListKey = (title?: string) => {
    dialog.create({
      title: title ?? 'New Key',
      content: () => {
        return <NewListForm></NewListForm>
      },
      draggable: true,
    })
  }

  const addHashKey = (title?: string) => {
    dialog.create({
      title: title ?? 'New Key',
      content: () => {
        return <NewHashForm></NewHashForm>
      },
      draggable: true,
    })
  }

  const addSetKey = (title?: string) => {
    dialog.create({
      title: title ?? 'New Key',
      content: () => {
        return <NewSetForm></NewSetForm>
      },
      draggable: true,
    })
  }
  const addZSetKey = (title?: string) => {
    dialog.create({
      title: title ?? 'New Key',
      content: () => {
        return <div>good job</div>
      },
      draggable: true,
    })
  }

  return {
    options,
    addStringKey,
    addListKey,
    addHashKey,
    addSetKey,
    addZSetKey,
  }
}
