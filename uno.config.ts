// uno.config.ts
import {defineConfig} from 'unocss'
import presetUno from "@unocss/preset-uno"
import presetIcons from "@unocss/preset-icons"

export default defineConfig({
    presets: [
        presetUno(), // 样式预设方案
        presetIcons({
            // 图标处理器
            extraProperties: {// 图标样式
                display: "inline-block",
                "vertical-align": "middle"
            }
        })
    ],
    rules: [
        [/^m-([\.\d]+)$/, ([_, num]) => ({margin: `${num}px`})],
        [/^p-([\.\d]+)$/, ([_, num]) => ({padding: `${num}px`})],
        [/^h-([\.\d]+)$/, ([_, num]) => ({height: `${num}px`})],
        [/^w-([\.\d]+)$/, ([_, num]) => ({width: `${num}px`})],
        [/^left-([\.\d]+)$/, ([_, num]) => ({left: `${num}px`})],
        [/^top-([\.\d]+)$/, ([_, num]) => ({top: `${num}px`})],
        [/^right-([\.\d]+)$/, ([_, num]) => ({right: `${num}px`})],
        [/^bottom-([\.\d]+)$/, ([_, num]) => ({bottom: `${num}px`})],
        [/text-overflow/, () => ({
            "text-overflow": "ellipsis",
            overflow: "hidden",
            "white-space": "nowrap"
        })],
    ]
})