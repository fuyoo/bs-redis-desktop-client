declare global {
  interface Window {
    PKG: PKG
  }
}

declare class PKG {
  name: string
  version: string
  description: string
  author: string
  license: string
  repository: string
  bugs: string
  homepage: string
  private: boolean
  main: string
  scripts: {
    [key: string]: string
  }
  devDependencies: {
    [key: string]: string
  }
  dependencies: {
    [key: string]: string
  }
}
