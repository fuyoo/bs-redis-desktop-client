<template>
  <el-config-provider :locale="locale">
    <router-view></router-view>
  </el-config-provider>
</template>

<script>
import zh from 'element-plus/lib/locale/lang/zh-cn'
import en from "element-plus/lib/locale/lang/en"
import {mapGetters} from "vuex"

export default {
  name: 'Layout',
  computed: {
    ...mapGetters(["i18n", "lang"]),
    locale() {
      switch (this.lang) {
        case "en":
          return en
        case "zh":
          return zh
        default:
          return zh
      }
    }
  },
}
</script>

<style lang="scss">

body {
  margin: 0;
  padding: 0;
  overflow: hidden;
}

body::before {
  content: "";
  position: fixed;
  left: 0;
  top: 0;
  width: 100%;
  height: 1px;
  background-color: $border;
  z-index: 9999999999999;
}
</style>
