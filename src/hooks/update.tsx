import { useDialog, NProgress } from 'naive-ui'
import { message } from '@/tools'
import { check } from '@tauri-apps/plugin-updater'
import { useI18n } from 'vue-i18n'
import { getVersion } from '@tauri-apps/api/app'
import { relaunch } from '@tauri-apps/plugin-process'
import { warn } from '@tauri-apps/plugin-log'
export const useUpdate = () => {
  const dialog = useDialog()
  const { t } = useI18n()
  const checkUpdate = async () => {
    return await check()
  }
  const askUpdate = async () => {
    const version = await getVersion()
    const update = await checkUpdate()
    console.log(update, version)
    if (!update) {
      return
    }
    dialog.destroyAll()
    const d = dialog.create({
      title: t('update.title'),
      draggable: true,
      content: () => {
        return update.body
      },
      negativeText: t('actions.1'),
      positiveText: t('actions.15'),
      onPositiveClick: () => {
        d.loading = true
        return new Promise((resolve, reject) => {
          let contentLength = 0
          let downloaded = 0
          // at here we can download the update
          update
            .download((evt) => {
              switch (evt.event) {
                case 'Started':
                  contentLength = evt.data.contentLength || 0
                  d.content = () => <NProgress type="line" status="success" percentage={0} />
                  break
                case 'Progress':
                  downloaded += evt.data.chunkLength
                  d.content = () => (
                    <NProgress
                      type="line"
                      status="success"
                      percentage={Math.floor((downloaded / contentLength) * 100)}
                    />
                  )
                  break
                case 'Finished':
                  d.content = () => <NProgress type="line" status="success" percentage={100} />
                  resolve('ok')
                  dialog.success({
                    draggable: true,
                    title: t('update.install'),
                    content: t('tips.10'),
                    positiveText: t('actions.16'),
                    negativeText: t('actions.1'),
                    onPositiveClick: async () => {
                      try {
                        await update.install()
                        await relaunch()
                      } catch (e: any) {
                        warn(e)
                        message.warning(e)
                      }
                    },
                  })
                  break
              }
            })
            .catch((e) => {
              reject(e)
            })
        })
      },
    })
  }
  return {
    checkUpdate,
    askUpdate,
  }
}
