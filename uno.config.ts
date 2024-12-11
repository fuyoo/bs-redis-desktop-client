import { defineConfig, presetIcons, presetUno } from 'unocss'

export default defineConfig({
  presets: [
    presetUno(), // 样式预设方案
    presetIcons({
      // 图标处理器参数
      extraProperties: {
        // 图标样式
        display: 'inline-block',
        'vertical-align': 'middle',
      },
    }),
  ],
})
