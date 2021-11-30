<template>
  <div class="list">
    <div class="key-data-title">
      <span><span class="highlight">{{ k }}</span> 查询结果</span>
      <div style="display: flex">
        <el-button size="mini" type="primary" @click="addNewListItemFn">
          <icon name="add-fill"></icon>
          新增
        </el-button>
        <el-button size="mini" type="primary" @click="sliceFn">
          <icon name="scissors-cut-line"></icon>
          裁剪
        </el-button>
      </div>
    </div>
    <el-scrollbar class="key-data-scroller border" style="width: 100%;" v-loading="dataLoading">
      <div style="position: absolute;left: 0;right: 0;top: 0;bottom: 60px">
        <el-table :data="tableData" style="width: 100%;" height="100%">
          <el-table-column label="#" width="60px">
            <template #default="scope">
              <span>{{ (pager.page - 1) * pager.size + Number(scope.$index) }}</span>
            </template>
          </el-table-column>
          <el-table-column :show-overflow-tooltip="true" label="数据">
            <template #default="{row}">
              <span>{{ row }} </span>
            </template>
          </el-table-column>
          <el-table-column label="操作" width="140px">
            <template #default="scope">
              <el-button type="text" @click="modifyFn(scope.$index + pager.size * ( pager.page - 1 ))">修改</el-button>
              <el-divider direction="vertical"></el-divider>
              <el-button type="text" :loading="deleteLoading" @click="deleteFn(scope.row)">删除</el-button>
              <el-divider direction="vertical"></el-divider>
              <el-button type="text" @click="lookFn(scope.row)">查看</el-button>
            </template>
          </el-table-column>
        </el-table>
      </div>
      <div style="width: 0;position: absolute;bottom: 10px">
        <el-pagination background @current-change="currenChangeFn" @size-change="sizeChangeFn"
                       layout="total, sizes, prev, pager, next, jumper"
                       :total="pager.total"></el-pagination>
      </div>
    </el-scrollbar>
    <el-dialog title="修改" :close-on-click-modal="false" width="380px" v-model="modifyVisible">
      <el-form label-width="80px" :model="modifyFormData" ref="modifyForm">
        <el-form-item label="新值" prop="value" :rules="{required: true,message:`新值不能为空`}">
          <el-input v-model="modifyFormData.value" placeholder="请输入新值"></el-input>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="modifyOkFn" :loading="modifyLoading">确定</el-button>
          <el-button type="danger" @click="modifyVisible = false" :loading="modifyLoading">取消</el-button>
        </el-form-item>
      </el-form>
    </el-dialog>
    <el-dialog title="添加" :close-on-click-modal="false" width="380px" v-model="addVisible">
      <el-form label-width="80px" :model="addFormData" ref="addForm">
        <el-form-item label="值" prop="value" :rules="{required: true,message:`值不能为空`}">
          <el-input v-model="addFormData.value" placeholder="请输入值" clearable></el-input>
        </el-form-item>
        <el-form-item label="插入位置" prop="how" :rules="{required: true,message:`方式不能为空`}">
          <el-select style="width: 100%" v-model="addFormData.how">
            <el-option label="顶部" value="l"></el-option>
            <el-option label="尾部" value="r"></el-option>
          </el-select>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="addOkFn" :loading="addLoading">确定</el-button>
          <el-button type="danger" @click="addVisible = false" :loading="addLoading">取消</el-button>
        </el-form-item>
      </el-form>
    </el-dialog>
    <el-dialog title="裁剪" :close-on-click-modal="false" width="380px" v-model="sliceVisible">
      <el-form label-width="80px" :model="sliceFormData" ref="sliceForm">
        <el-form-item label="开始位置" prop="start" :rules="{required: true,message:`开始位置不能为空`,trigger:'blur'}">
          <el-input v-model="sliceFormData.start" placeholder="请输入开始位置"></el-input>
        </el-form-item>
        <el-form-item label="结束位置" prop="end" :rules="{required: true,message:`结束位置不能为空`,trigger:'blur'}">
          <el-input v-model="sliceFormData.end" placeholder="请输入结束位置"></el-input>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="sliceOkFn" :loading="sliceLoading">确定</el-button>
          <el-button type="danger" @click="sliceVisible = false" :loading="sliceLoading">取消</el-button>
        </el-form-item>
      </el-form>
    </el-dialog>
  </div>
</template>

<script>
import {invoke} from '@tauri-apps/api/tauri';
import {request} from ':/tools/invoke';
import {WebviewWindow} from '@tauri-apps/api/window';
import {listen} from '@tauri-apps/api/event';

export default {
  name: 'list',
  props: {
    k: {
      type: String,
      required: true
    }
  },
  data() {
    return {
      dataLoading: false,
      tableData: [],
      pager: {
        page: 1,
        total: 0,
        size: 10
      },
      modifyVisible: false,
      modifyLoading: false,
      modifyFormData: {
        index: 0,
        value: ''
      },
      deleteLoading: false,
      addVisible: false,
      addLoading: false,
      addFormData: {
        how: 'r',
        value: ''
      },
      sliceVisible: false,
      sliceLoading: false,
      sliceFormData: {
        start: '',
        end: ''
      }
    }
  },
  mounted() {
    this.refresh()
  },
  methods: {
    refresh() {
      this.fetchListData()
    },
    fetchListData() {
      this.dataLoading = true
      const params = {
        key: this.k,
        page: this.pager.page,
        size: this.pager.size
      }
      invoke('get_list_data', params)
          .then(res => {
            const {page, size, total} = res.data
            this.pager.size = size
            this.pager.total = total
            this.pager.page = page
            this.tableData = res.data.records
          })
          .finally(() => this.dataLoading = false)
    },
    currenChangeFn(p) {
      this.pager.page = p
      this.fetchListData()
    },
    sizeChangeFn(s) {
      this.pager.size = s
      this.fetchListData()
    },
    modifyFn(index) {
      this.modifyFormData = {
        index,
        value: '',
        key: this.k
      }
      this.modifyVisible = true
    },
    modifyOkFn() {
      this.$refs.modifyForm.validate(valid => {
        if (!valid) return
        this.modifyLoading = true
        invoke('modify_list_data_by_index', this.modifyFormData)
            .then(res => {
              if (res.code === 200) {
                this.alert.success(res.msg)
                this.modifyVisible = false
                this.fetchListData()
                return
              }
              this.alert.error(res.msg)
            })
            .finally(() => this.modifyLoading = false)
      })
    },
    deleteFn(value) {
      this.$confirm('删除后无法恢复，是否继续？')
          .then(() => {
            this.deleteLoading = true
            invoke('delete_list_data_by_value', {key: this.k, value})
                .then(res => {
                  if (res.code === 200) {
                    this.alert.success(res.msg)
                    this.fetchListData()
                    return
                  }
                  this.alert.error(res.msg)
                })
                .finally(() => this.deleteLoading = false)
          })
          .catch()
    },
    addNewListItemFn() {
      this.addVisible = true
      this.addFormData = {
        key: this.k,
        value: '',
        how: 'r'
      }
    },
    addOkFn() {
      this.$refs.addForm.validate(valid => {
        if (!valid) return
        this.addLoading = true
        invoke('push_data_to_exists_key_list', this.addFormData)
            .then(res => {
              if (res.code === 200) {
                this.alert.success(res.msg)
                this.addVisible = false
                this.fetchListData()
                return
              }
              this.alert.error(res.msg)
            })
            .finally(() => this.addLoading = false)
      })
    },
    sliceFn() {
      this.sliceVisible = true
      this.sliceFormData = {
        key: this.k,
        start: '',
        end: ''
      }
    },
    sliceOkFn() {
      this.$refs.sliceForm.validate(valid => {
        if (!valid) return
        this.sliceLoading = true
        let data = {
          key: this.k,
          start: Number(this.sliceFormData.start),
          end: Number(this.sliceFormData.end)
        }
        request('slice_list', data)
            .then(res => {
              if (res.code === 200) {
                this.alert.success(res.msg)
                this.sliceVisible = false
                this.fetchListData()
                return
              }
              this.alert.error(res.msg)
            })
            .finally(() => this.sliceLoading = false)
      })
    },
    lookFn(row) {
      let w = new WebviewWindow(Math.random().toString(36).slice(2), {
        url: '/#/look-more',
        title: '数据详情',
        alwaysOnTop: true
      });
      w.listen('data', () => {
        w.emit('data', row)
      })
    }
  }
}
</script>

<style scoped lang="scss">
@include common-title;
</style>
