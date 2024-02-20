import { PakeCliOptions } from './types.js';

export const DEFAULT_PAKE_OPTIONS: PakeCliOptions = {
  icon: '',
  height: 780,
  width: 1200,
  fullscreen: false,
  resizable: true,
  transparent: false,
  userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36',
  showMenu: false,
  showSystemTray: false,
  multiArch: false,
  targets: 'deb',
  iterCopyFile: false,
  systemTrayIcon: '',
  debug: false,
  inject: [],
  safeDomain: [],
};

// Just for cli development
export const DEFAULT_DEV_PAKE_OPTIONS: PakeCliOptions & {url: string} = {
  ...DEFAULT_PAKE_OPTIONS,
  url: 'https://weread.qq.com',
  name: 'WeRead',
  safeDomain:['weread.qq.com'],
  transparent: true,
}
