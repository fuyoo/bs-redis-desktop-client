import pluginVue from 'eslint-plugin-vue'
import vueTsEslintConfig from '@vue/eslint-config-typescript'
import pluginVitest from '@vitest/eslint-plugin'
import oxlint from 'eslint-plugin-oxlint'
import skipFormatting from '@vue/eslint-config-prettier/skip-formatting'

export default [
  {
    name: 'app/files-to-lint',
    files: ['**/*.{ts,mts,tsx,vue}'],
  },

  {
    name: 'app/files-to-ignore',
    ignores: ['**/dist/**', '**/dist-ssr/**', '**/coverage/**'],
  },

  ...pluginVue.configs['flat/essential'],
  ...vueTsEslintConfig(),

  {
    ...pluginVitest.configs.recommended,
    files: ['src/**/__tests__/*'],
  },
  oxlint.configs['flat/recommended'],
  skipFormatting,
  {
    rules: {
      'prefer-promise-reject-errors': 'off',
      '@typescript-eslint/consistent-type-imports': ['error', { prefer: 'type-imports' }],
      '@typescript-eslint/no-unused-expressions': 'off',
      'vue/multi-word-component-names': 'off',
      '@typescript-eslint/no-explicit-any': 'off',
      // allow debugger during development only
      'no-debugger': process.env.NODE_ENV === 'production' ? 'error' : 'off',
    },
  },
]
