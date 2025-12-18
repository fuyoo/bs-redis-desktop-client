import NewStringForm from '@/pages/host/components/CoKeys/compoents/NewStringForm.vue'
import NewSetForm from '@/pages/host/components/CoKeys/compoents/NewSetForm.vue'
import NewZSetForm from '@/pages/host/components/CoKeys/compoents/NewZSetForm.vue'
import NewListForm from '@/pages/host/components/CoKeys/compoents/NewListForm.vue'
import NewHashForm from '@/pages/host/components/CoKeys/compoents/NewHashForm.vue'
import type { DialogApiInjection } from 'naive-ui/es/dialog/src/DialogProvider'
import { useReqStore } from '@/stores/req'
import { useI18n } from 'vue-i18n'
import { message } from '@/tools'
import { useRoute, useRouter } from 'vue-router'

export const useActions = (dialog: DialogApiInjection) => {
  const req = useReqStore()
  const { t } = useI18n()
  const router = useRouter()
  const route = useRoute()
  const checkKeyIsExist = async (key: string, hideMsg?: boolean, jump?: boolean) => {
    const resp = await req.reqWithHost({
      path: `/cmd`,
      data: ['exists', key],
    })
    if (resp.data === '0') {
      // if key is not exist and jump is true, jump to new key page
      if (jump) {
        await router.replace(`/tab/${route.params.id}/main/database/none/${btoa(key)}`)
        return
      }
      return Promise.resolve(false)
    }
    if (!hideMsg) {
      message.destroyAll()
      message.warning(t('tips.7', { key }))
    }

    return Promise.reject(t('tips.7', { key }))
  }
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
        return <NewZSetForm></NewZSetForm>
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
    checkKeyIsExist,
  }
}
