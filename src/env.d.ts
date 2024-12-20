declare namespace NodeJS {
  interface ProcessEnv {
    NODE_ENV: string
    VUE_ROUTER_MODE: 'hash' | 'history' | 'abstract' | undefined
    VUE_ROUTER_BASE: string | undefined
  }
}

declare namespace global {
  interface Window {
    package: Package
  }
}

declare class Package {
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
  name: string
}
