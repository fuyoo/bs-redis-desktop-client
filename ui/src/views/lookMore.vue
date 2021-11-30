<template>
  <div>
    <div class="control" @click.stop="toggle">
      <icon :name="isShow ? 'close-fill' : 'pantone-fill'"></icon>
    </div>
    <div @click.stop class="menu" :class="{focus:isShow}">
      <div class="menu-title">
        格式化方式
      </div>
      <div class="item" @click="changeTypeFn('string')" :class="{'item-active':type === 'string'}">
        <icon name="text"></icon>
        <span class="text">string</span>
      </div>
      <div class="item" @click="changeTypeFn('json')" :class="{'item-active':type === 'json'}">
        <icon name="brackets-fill"></icon>
        <span class="text">json</span>
      </div>
      <div class="item" @click="changeTypeFn('blob')" :class="{'item-active':type === 'blob'}">
        <icon name="stack-line"></icon>
        <span class="text">blob</span>
      </div>
    </div>
    <el-scrollbar class="look-more-scrollbar">
      <div class="data" v-if="type === 'string'">{{ this.value }}</div>
      <div class="data" v-if="type === 'json'">
        <json-viewer :json="jsonData"></json-viewer>
      </div>
      <div class="data" v-if="type === 'blob'"> {{ blobData }}</div>
    </el-scrollbar>
  </div>
</template>

<script>
import {emit, listen} from '@tauri-apps/api/event'
import {JsonViewer} from ':/components/JsonViewer'

export default {
  name: 'lookMore',
  components: {
    JsonViewer
  },
  data() {
    return {
      isShow: false,
      type: 'string',
      value: ''
    }
  },
  computed: {
    jsonData() {
      try {
        return JSON.parse(this.value)
      } catch (e) {
        return {
          error: e.toString(),
          message: 'json格式化错误！'
        }
      }
    },
    blobData() {
      let res = []
      try {
        let val = JSON.parse(this.value)
        val.forEach(item => {
          res.push(String.fromCharCode(item))
        })
      } catch (e) {
        return 'blob解析失败！'
      }
      return res.join('')
    }
  },
  methods: {
    toggle() {
      this.isShow = !this.isShow
    },
    changeTypeFn(t) {
      this.type = t
    }
  },
  mounted() {
    emit('data', JSON.stringify({type: 1}))
    listen('data', event => {
      this.value = event.payload
    })
    document.onclick = () => {
      this.isShow = false
    }
  }
}
</script>

<style scoped lang="scss">
.control {
  background: $primary-color;
  display: flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
  position: fixed;
  right: 5px;
  top: 5px;
  color: #fff;
  box-shadow: 0 0 5px $border;
  border-radius: 4px;
  cursor: pointer;
  user-select: none;
  -webkit-user-select: none;
  transition: .168s;
  z-index: 2;

  &:active {
    opacity: 0.7;
  }
}

.menu {
  width: 120px;
  position: fixed;
  right: 5px;
  top: 40px;
  box-shadow: 0 0 5px $border;
  transition: .168s;
  border-radius: 4px 4px 0 0;
  overflow: hidden;
  transform: translateX(200px);
  z-index: 2;

  user-select: none;
  -webkit-user-select: none;

  .menu-title {
    background: $primary-color;
    height: 30px;
    line-height: 30px;
    color: #fff;
    font-size: 13px;
    text-align: center;
  }

  .item {
    padding-left: 30px;
    height: 30px;
    text-align: center;
    line-height: 30px;
    font-size: 12px;
    display: flex;
    justify-content: flex-start;
    align-items: center;
    position: relative;
    transition: .168s;
    cursor: pointer;

    &:hover {
      background: #eee;
    }
  }

  .item .text {
    margin-left: 8px;
  }

  .item + .item {
    border-top: 1px solid #eee;
  }

  .item-active {
    color: $primary-color;

    &::before {
      content: '';
      position: absolute;
      background: url("~:/asserts/icon/check-line.svg") center center / 35% no-repeat;
      left: 0;
      top: 0;
      width: 30px;
      height: 30px;
    }
  }
}

.focus {
  transform: translateX(0);
}

.look-more-scrollbar {
  position: absolute;
  left: 0;
  top: 0px;
  width: 100%;
  height: 100%;

  .data {
    padding: 15px;
  }
}
</style>