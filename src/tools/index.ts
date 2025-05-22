import { Window } from '@tauri-apps/api/window'
import { Webview } from '@tauri-apps/api/webview'

export const showHostConfigureDetail = async (id: string) => {
  const appWindow = new Window('host-config-detail', {
    title: 'BS',
  })

  // loading embedded asset:
  const webview = new Webview(appWindow, 'host-config-detail', {
    url: '/#/detail/host/' + id,
  } as any)
  setTimeout(() => {
    webview?.setFocus()
  }, 10)
}

export const dataToHuman = (data: string): string => {
  const bytes = parseInt(data, 10);
  console.log(bytes);
  if (isNaN(bytes) || bytes < 0) return '0 B';

  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
  const index = Math.floor(Math.log(bytes) / Math.log(1024));
  const size = bytes / Math.pow(1024, index);

  return `${size.toFixed(0)} ${sizes[index]}`;
};
