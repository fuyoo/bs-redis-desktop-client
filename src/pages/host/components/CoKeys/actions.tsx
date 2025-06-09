
import type {DialogApiInjection} from "naive-ui/es/dialog/src/DialogProvider";
import NewStringForm from "@/pages/host/components/CoKeys/compoents/NewStringForm.vue";
export const options = [{
  label: 'string',
  value: 'string'
}, {
  label: 'set',
  value: 'set'
}, {
  label: 'hash',
  value: 'hash'
}, {
  label: 'list',
  value: 'list'
}, {
  label: 'zset',
  value: 'zset'
},]

const template = {
  draggable: true,
  maskClosable: false,
}
export const addStringKey = (dialog: DialogApiInjection) => {
  dialog.create({
    ...template,
    title: "New",
    content: () => {
      return <NewStringForm></NewStringForm>
    }
  })
}

export const addListKey = (dialog: DialogApiInjection) => {
  dialog.create({
    content: () => {

      return <div>good job</div>
    },
    draggable: true
  })
}


export const addHashKey = (dialog: DialogApiInjection) => {
  dialog.create({
    content: () => {

      return <div>good job</div>
    },
    draggable: true
  })
}

export const addSetKey = (dialog: DialogApiInjection) => {
  dialog.create({
    content: () => {

      return <div>good job</div>
    },
    draggable: true
  })
}
export const addZSetKey = (dialog: DialogApiInjection) => {
  dialog.create({
    content: () => {

      return <div>good job</div>
    },
    draggable: true
  })
}
