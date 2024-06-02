<script setup lang="ts">

</script>

<template>
  <div class="p-25px">
    <a-form auto-label-width :model="{}">
      <a-card class="rounded-lg" hoverable :body-style="{padding:'10px 15px 0 15px'}">
        <template #title>
          <span>{{$t('layout.hostForm.name')}}</span>
        </template>
        <a-form-item required>
          <template #label>
            <div class="i-tabler:server-bolt text-24px"></div>
          </template>
          <a-input :placeholder="$t('layout.hostForm.hostName.placeholder')"></a-input>
        </a-form-item>
      </a-card>
      <br>
      <a-card :title="$t('layout.hostForm.normal')" hoverable>
        <a-form-item required>
          <template #label>
            <div class="i-iconoir:network-left-solid"></div>
          </template>
          <a-input :placeholder="$t('layout.hostForm.host.placeholder')"></a-input>
        </a-form-item>
        <a-form-item>
          <template #label>
            <div class="i-material-symbols:power-plug"></div>
          </template>
          <a-input :placeholder="$t('layout.hostForm.port.placeholder')"></a-input>
        </a-form-item>
        <a-form-item>
          <template #label>
            <div class="i-ic:round-sd-storage"></div>
          </template>
          <a-input :placeholder="$t('layout.hostForm.db.placeholder')"></a-input>
        </a-form-item>
        <a-form-item>
          <template #label>
            <div class="i-ic:round-person"></div>
          </template>
          <a-input :placeholder="$t('layout.hostForm.username.placeholder')"></a-input>
        </a-form-item>
        <a-form-item>
          <template #label>
            <div class="i-ic:round-lock"></div>
          </template>
          <a-input-password :placeholder="$t('layout.hostForm.password.placeholder')"></a-input-password>
        </a-form-item>
      </a-card>
    </a-form>

  </div>
</template>

<style scoped lang="scss">
.p-25px {
  &::v-deep(.arco-form-item-label-col) {
    // display: none;
  }
}
</style>