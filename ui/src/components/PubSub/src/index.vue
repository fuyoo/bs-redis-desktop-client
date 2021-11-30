<template>
  <div class="pubsub">
    <div class="action-bar">
      <div style="width: 10px"></div>
      <el-button type="primary" @click="newListenerFn">
        <icon name="add-line"></icon>
        新增订阅
      </el-button>
      <el-button type="primary" @click="getChannelList">
        <icon name="refresh-line"></icon>
        刷新列表
      </el-button>
      <el-button type="primary" @click="publishDataFn">发布数据</el-button>
    </div>
    <div class="listener">
      <div class="ls-title">
        <icon name="list-check" style="margin-right: 3px"></icon>
        订阅列表
      </div>
      <el-scrollbar class="ls-scroller" height="100%" v-if="channelList.length > 0">
        <div class="ls-item" :class="{running:item.stop === 'false'}" v-for="item in channelList">
          <icon name="router-line"></icon>
          {{ item.key }}
          <span class="l-stop" @click="close(item.key)"><icon name="stop-mini-fill"></icon></span>
        </div>
      </el-scrollbar>
      <el-empty description="暂无订阅列表" v-else></el-empty>
    </div>
    <div class="results">
      <div class="filter">
        <span><icon name="reserved-line" style="margin-right: 3px"></icon>订阅结果</span>
        <div class="filter-input">
          <input class="input" @change="filterChangeFn" type="text" v-model="filterData" placeholder="过滤条件"/>
          <span class="s-icon" @click="searchFn">
            <icon name="filter--line"></icon>
          </span>
        </div>
      </div>
      <el-scrollbar ref="ctxScrollbar" class="res-scroller">
        <div class="r-ctx" v-if="pubsubResult.length > 0">
          <div class="r-ctx-item" v-for="v in pubsubResult">
            <div class="r-ctx-item-channel">
              <span> <icon name="router-line"></icon> {{ v.key }}</span>
              <span><icon name="map-pin-time-line"></icon> {{ v.time }}</span>
            </div>
            <div class="r-ctx-item-val">
              <div v-if="isJson(v.data)">
                <json-viewer :json="formatJsonData(v.data)"></json-viewer>
              </div>
              <div v-else>
                <icon name="exchange-box-fill"></icon>
                {{ v.data }}
              </div>
            </div>
          </div>
        </div>
        <el-empty description="暂无订阅数据" v-else></el-empty>
      </el-scrollbar>
    </div>
    <el-dialog title="新增订阅通道" v-model="newListener" :close-on-click-modal="false" width="380px">
      <el-form ref="form" label-width="80px" :model="form">
        <el-form-item prop="name" :rules="{required:true,message:'通道名称不能为空',trigger:'blur'}" label="通道名称">
          <el-input type="form" placeholder="请输入通道名称" v-model="form.name"></el-input>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" :loading="newListenerLoading" @click="okFn">提交</el-button>
          <el-button type="primary" :loading="newListenerLoading" @click="newListener = false">取消</el-button>
        </el-form-item>
      </el-form>
    </el-dialog>
    <el-dialog title="发布消息" v-model="publishMsg" :close-on-click-modal="false" width="380px">
      <el-form ref="pubForm" label-width="80px" :model="publishForm">
        <el-form-item prop="channel" :rules="{required:true,message:'通道名称不能为空',trigger:'blur'}" label="通道名称">
          <el-input placeholder="请输入通道名称" v-model="publishForm.channel"></el-input>
        </el-form-item>
        <el-form-item prop="data" :rules="{required:true,message:'数据不能为空',trigger:'blur'}" label="数据">
          <el-input type="form" placeholder="请输入通道名称" v-model="publishForm.data"></el-input>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" :loading="pubLoading" @click="pubOkFn">提交</el-button>
          <el-button type="primary" :loading="pubLoading" @click="publishMsg = false">取消</el-button>
        </el-form-item>
      </el-form>
    </el-dialog>
  </div>
</template>

<script>
import {request} from ':/tools/invoke';
import {mapGetters} from 'vuex'
import {JsonViewer} from ':/components/JsonViewer';

export default {
  name: 'PubSub',
  components: {
    JsonViewer
  },
  data() {
    return {
      channelList: [],
      newListener: false,
      newListenerLoading: false,
      filterData: '',
      searchData: '',
      form: {
        name: ''
      },
      pubLoading: false,
      publishMsg: false,
      publishForm: {}
    }
  },
  computed: {
    pubsubResult() {
      let res = []
      if (this.searchData != '') {
        res = this.pubsub.filter(item => {
          if (item.key.indexOf(this.searchData) > -1 || item.data.indexOf(this.searchData) > -1) {
            return true
          } else {
            return false
          }
        })
      } else {
        res = this.pubsub
      }
      setTimeout(() => {
        this.$nextTick = () => {
          this.$refs.ctxScrollbar.setScrollTop(99999999)
        }
      })
      return res
    },
    ...mapGetters(['pubsub'])
  },
  mounted() {
    this.getChannelList()
  },
  methods: {
    getChannelList() {
      request('get_channel_list')
          .then(res => {
            this.channelList = res.data
          })
    },
    close(channel) {
      request('remove_channel', {channel})
          .then(() => {
            this.getChannelList()
          })
    },
    newListenerFn() {
      this.form = {}
      this.newListener = true
    },
    okFn() {
      this.$refs.form.validate(valid => {
        if (!valid) return
        this.newListenerLoading = true
        request('create_channel', this.form)
            .then(res => {
              this.alert.success(res.msg)
              this.getChannelList();
              this.newListener = false
            })
            .finally(() => this.newListenerLoading = false)
      })
    },
    filterChangeFn() {
      if (this.filterData == '') {
        this.searchData = ''
      }
    },
    searchFn() {
      this.searchData = this.filterData
    },
    isJson(data) {

      try {
        if (data.indexOf('[') > -1 || data.indexOf('{') > -1) {
          JSON.parse(data)
          return true
        }
        return false
      } catch (e) {
        return false
      }
    },
    formatJsonData(data) {
      return JSON.parse(data)
    },
    publishDataFn() {
      this.publishMsg = true
      this.publishForm = {}
    },
    pubOkFn() {
      this.$refs.pubForm.validate(valid => {
        if (!valid) return;
        this.pubLoading = true
        request('publish_msg', this.publishForm)
            .then(res => {
              this.alert.success(res.msg)
              this.publishMsg = false
            })
            .finally(() => this.pubLoading = false)
      })
    },

  }
}

</script>

<style scoped lang="scss">
.pubsub {
  position: relative;

  .action-bar {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    height: 35px;
    width: 100%;
    border-bottom: 1px solid $border;

    ::v-deep(.el-button--mini) {
      font-size: 10px;
      min-height: 22px;
      padding: 0 8px;
    }
  }

  .listener {
    width: 200px;
    position: absolute;
    left: 0;
    top: 36px;
    height: calc(100vh - 68px);
    border-right: 1px solid $border;
    -webkit-user-select: none;
    user-select: none;

    .ls-title {
      height: 30px;
      border-bottom: 1px solid $border;
      display: flex;
      justify-content: flex-start;
      align-items: center;
      font-size: 13px;
      text-indent: 10px;
    }

    .ls-scroller {
      height: calc(100% - 31px);
    }

    .ls-item {
      font-size: 12px;
      padding: 0 10px;
      position: relative;
      height: 30px;
      line-height: 30px;

      .l-stop {
        position: absolute;
        right: 15px;
        top: 50%;
        margin-top: -10px;
        width: 20px;
        height: 20px;
        display: flex;
        justify-content: center;
        align-items: center;
        font-size: 16px;
        opacity: 0;
        transition: .168s;
        cursor: pointer;

        &:active {
          opacity: 0.2 !important;
        }
      }

      &:hover > .l-stop {
        opacity: 1;
      }
    }

    .ls-item + .ls-item {
      border-top: 1px dashed $border;
    }

    .running::before {
      content: '';
      position: absolute;
      right: 5px;
      top: 50%;
      margin-top: -3px;
      height: 6px;
      width: 6px;
      background: $primary-color;
      border-radius: 50%;
    }
  }

  .results {
    position: absolute;
    left: 200px;
    right: 0;
    top: 36px;
    height: calc(100vh - 68px);

    .filter {
      height: 30px;
      border-bottom: 1px solid $border;
      display: flex;
      justify-content: space-between;
      align-items: center;
      font-size: 13px;
      padding: 0 10px;
      -webkit-user-select: none;
      user-select: none;

      .filter-input {
        border-radius: 3px;
        padding: 0;
        display: flex;
        justify-content: center;
        align-items: center;

        .input {
          outline: none;
          border: none;
          width: 120px;
          padding: 3px 0;
        }

        .s-icon {
          display: flex;
          width: 20px;
          height: 20px;
          justify-content: center;
          align-items: center;
          line-height: 1;
          cursor: pointer;
          transition: .168s;

          &:active {
            opacity: 0.6;
          }
        }
      }
    }

    .res-scroller {
      height: calc(100vh - 100px);

      .r-ctx {
        .r-ctx-item {
          padding: 15px 15px 8px 15px;
          font-size: 12px;
          border-bottom: 1px dashed $border;

          .r-ctx-item-channel {
            padding-bottom: 6px;
            border-bottom: 1px dotted $border;
            display: flex;
            justify-content: space-between;
            align-items: center;
          }

          .r-ctx-item-val {
            padding: 8px 0;
          }
        }

      }
    }
  }
}
</style>