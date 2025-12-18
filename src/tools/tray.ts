import { t } from '@/i18n'
import { req } from '@/api'

export const buildTray = async () => {
  await req.do('init_tray', { text: t('tray.quit') })
}
