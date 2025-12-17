import type { PaginationInfo } from 'naive-ui'
import { reactive } from 'vue'
import { t } from '@/i18n'
import { NText } from 'naive-ui'
export const usePager = (obj?: { page?: number; pageSize?: number }) => {
  let cb = () => {}

  const pager = reactive({
    page: 1,
    pageSize: 10,
    pageCount: 0,
    itemCount: 0,
    showSizePicker: true,
    pageSizes: [10, 20, 50, 100],
    prefix({ itemCount }: PaginationInfo) {
      return <NText>{t('tips[4]', { size: itemCount })}</NText>
    },
    onChange: (page: number) => {
      pager.page = page
      cb()
    },
    onUpdatePageSize: (pageSize: number) => {
      pager.pageSize = pageSize
      pager.page = 1
      cb()
    },
    ...obj,
  })
  const onPageChanged = (func: () => void) => {
    cb = func
  }
  const calcIndex = (index: number) => {
    return (pager.page - 1) * pager.pageSize + index + 1
  }
  return {
    pager,
    onPageChanged,
    calcIndex,
  }
}
