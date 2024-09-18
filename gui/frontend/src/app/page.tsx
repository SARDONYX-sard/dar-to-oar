import Converter from '@/components/pages/converter';

/**
 * # Root page (URL: /).
 */
export default function Home() {
  // HACK: Avoid blank white screen on load.
  // - https://github.com/tauri-apps/tauri/issues/5170#issuecomment-2176923461
  // - https://github.com/tauri-apps/tauri/issues/7488
  if (typeof window !== 'undefined') {
    import('@tauri-apps/api').then((tauri) => {
      tauri.window.appWindow.show();
    });
  }

  return <Converter />;
}
