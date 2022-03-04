<template>
  <div class="string">
    <div class="key-data-title">
      <span><span class="highlight">{{ k }}</span> {{ lang.title }}</span>
      <div>{{ lang.format }}：
        <el-select style="width: 100px" v-model="dataType" :placeholder="lang.formatPlaceholder">
          <el-option label="string" value="string"></el-option>
          <el-option label="json" value="json"></el-option>
          <el-option label="blob" value="blob"></el-option>
        </el-select>
      </div>
    </div>
    <el-scrollbar style="width: 100%" class="key-data-scroller border" v-loading="dataLoading">
      <div class="pre" v-if="dataType === 'string'">
        {{ this.content }}
      </div>
      <div class="pre" style="margin: 15px 0;width: 0;" v-if="dataType === 'json'">
        <json-viewer font-size="14" :json="jsonData"></json-viewer>
      </div>
      <div class="pre" v-if="dataType === 'blob'">
        {{ blobData }}
      </div>
    </el-scrollbar>
  </div>
</template>

<script>
import {invoke} from '@tauri-apps/api/tauri';
import {JsonViewer} from ":/components/JsonViewer";
import {mapGetters} from "vuex";

export default {
  name: 'string',
  components: {
    JsonViewer
  },
  props: {
    k: {
      type: String,
      required: true
    }
  },
  data() {
    return {
      dataLoading: false,
      dataType: 'string',
      content: "",
    }
  },
  computed: {
    jsonData() {
      let obj = ""
      try {
        obj = JSON.parse(this.content)
      } catch (e) {
        obj = {"error": e.toString(), "message": "数据不是json格式！"}
      }
      return obj
    },
    blobData() {
      let res = []
      let len = this.content.length
      for (let item = 0; item < len; item++) {
        try {
          res.push(String.fromCharCode(this.content[item]))
        } catch (e) {
          res.push('?')
        }
      }
      return res.join('')
    },
    ...mapGetters(['i18n']),
    lang() {
      return this.i18n.mainPage.content.dataPage.string
    }
  },
  watch: {
    dataType(r) {
      if (!r) {

        this.fetchData()
      }
    }
  },
  created() {
    this.fetchData()
  },
  methods: {
    refresh() {
      this.fetchData()
    },
    fetchStringData() {
      this.dataLoading = true
      invoke('get_string_type_data_as_string', {key: this.k})
          .then(res => {
            if (res.code !== 200) {
              this.alert.error(res.msg)
            } else {
              this.content = res.data
            }
          })
          .finally(() => this.dataLoading = false)

    },
    fetchBlobData() {
      this.dataLoading = true
      invoke('get_string_type_data_as_blob', {key: this.k})
          .then(res => {
            if (res.code !== 200) {
              this.alert.error(res.msg)
            } else {
              this.content = res.data
            }
          })
          .finally(() => this.dataLoading = false)
    },
    fetchData() {
      this.content = ''
      switch (this.dataType) {
        case 'string':
          this.fetchStringData()
          break
        case 'blob':
          this.fetchBlobData()
          break
        default:
          this.fetchStringData()
      }
    },
  }
}
</script>

<style lang="scss" scoped>
.string {

}

@include common-title;
.border {
  border: 1px solid #eee;

  .pre {
    margin: 15px;
    word-break: break-all;
  }
}
</style>
