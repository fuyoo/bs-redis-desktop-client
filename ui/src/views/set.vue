<template>
  <el-scrollbar style="height: 100vh">
    <el-form class="form" ref="form" :model="form" label-width="120px">
      <el-form-item :label="i18n.setPage.autoRefresh">
        <el-switch v-model="form.autoRefresh"></el-switch>
      </el-form-item>
      <el-form-item :label="i18n.setPage.autoRefreshTime" v-if="form.autoRefresh">
        <el-input type="number" v-model="form.autoRefreshTime"
                  :placeholder="i18n.setPage.autoRefreshTimePlaceholder"></el-input>
      </el-form-item>
      <el-form-item :label="i18n.setPage.pubSub">
        <el-switch v-model="form.pubsub"></el-switch>
      </el-form-item>
      <el-form-item :label="i18n.setPage.lang">
        <el-select v-model="form.lang">
          <el-option label="简体中文" value="zh"></el-option>
          <el-option label="English" value="en"></el-option>
        </el-select>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="onSubmit">{{ i18n.setPage.save }}</el-button>
        <el-button @click="cancelFn">{{ i18n.setPage.cancel }}</el-button>
      </el-form-item>
    </el-form>
  </el-scrollbar>
</template>

<script>
import {request} from ':/tools/invoke';
import {getCurrent} from '@tauri-apps/api/window';
import {mapGetters} from "vuex";

export default {
  name: 'set',
  data() {
    return {
      form: {},
    }
  },
  computed: {
    ...mapGetters(['i18n'])
  },
  created() {
    getCurrent().show()
    this.getSysInfo()
  },
  mounted() {
  },
  methods: {
    getSysInfo() {
      request('query_sys_info')
          .then(res => {
            this.form = res.data || {id:"1"}
          })
    },
    onSubmit() {
      this.form.autoRefreshTime = Number(this.form.autoRefreshTime)
      request('update_sys_info', this.form)
          .then(res => {
            this.alert.success(res.msg)
          })
    },
    cancelFn() {
      getCurrent().close()
    }
  },
}
</script>

<style>
.el-message {
  min-width: 200px;
}
</style>
<style scoped lang="scss">


.form {
  padding: 20px;

}
</style>