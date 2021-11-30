<template>
  <div class="home">
    <database-menu :show="show"></database-menu>
    <div class="right-content">
      <content-body></content-body>
    </div>
  </div>
</template>

<script>
import {DatabaseMenu} from ':/components/DatabseMenu'
import {ContentBody} from ':/components/ContentBody'
import {getCurrent} from '@tauri-apps/api/window';

export default {
  name: 'Home',
  components: {
    DatabaseMenu,
    ContentBody
  },
  methods: {},
  mounted() {
    getCurrent().show()
  }
}
</script>

<style scoped lang="scss">
.home {
  display: flex;
  justify-content: flex-start;
  align-items: flex-start;

}

.right-content {
  flex: 1;
  height: 100vh;
}
</style>

