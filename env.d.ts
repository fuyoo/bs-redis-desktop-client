/// <reference types="vite/client" />

interface Package {
  name: string
  version: string
  description: string
  author: string
  license: string
  repository: string
  bugs: string
  homepage: string
  devDependencies: {
    [key: string]: string
  }
  dependencies: {
    [key: string]: string
  }
}

interface BackendResponse<T> {
  code: number
  data: T
  msg: string
}

interface Tab {
  id: string
  name?: string
}

declare const __APP_PKG: Package
