import {reactive} from 'vue'

export const usePager = (obj?: { page?: number; pageSize?: number }) => {
  let cb = () => {}

  const pager = reactive({
    page: 1,
    pageSize: 10,
    pageCount: 0,
    showSizePicker: true,
    pageSizes: [10, 20, 50, 100],
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
    return (pager.page - 1) * pager.pageSize + index+ 1
  }
  return {
    pager,
    onPageChanged,
    calcIndex
  }
}
