import { listen } from '@tauri-apps/api/event';
import { toast } from 'react-hot-toast';

type ListerProps = {
  setLoading: (loading: boolean) => void;
  setProgress: (percentage: number) => void;
  success: string | JSX.Element;
  error?: string | JSX.Element;
};

/**
 * Tauri Progress Event Listener.
 * # No exception
 *
 * # Example
 * ```typescript
 * const promiseFn = async() => {};
 * const setLoading = (bool:boolean) => {};
 * const setProgress = (per:number) => {};
 *
 * await progressListener('show-progress', promiseFn, {
 *    setLoading,
 *    setProgress,
 *    success: 'conversion-complete',
 * });
 * ```
 *
 * - backend(focus window.emit)
 *
 * ```rust
 * #[tauri::command]
 * pub(crate) async fn remove_oar_dir(window: Window, path: &str) -> Result<(), ()> {
 *     #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
 *     struct Payload { index: usize }
 *     let sender = |index:usize| window.emit("show-progress", Payload { index });
 *     sender(1);
 *     Ok(())
 * }
 *  ```
 */
export async function progressListener(eventName: string, promiseFn: () => Promise<void>, props: ListerProps) {
  const { setLoading, setProgress, success, error } = props;

  setLoading(true);
  let unlisten: (() => void) | null = null;

  try {
    setProgress(0);
    let maxNum = 0;
    let prog = 0;

    unlisten = await listen<{ index: number }>(eventName, (event) => {
      if (maxNum === 0) {
        maxNum = event.payload.index;
      } else {
        prog = event.payload.index;
      }
      setProgress((prog * 100) / maxNum);
    });

    await promiseFn();
    toast.success(success);
    setProgress(100);
  } catch (err) {
    setProgress(0); // To avoid NaN
    toast.error(error ?? `${err}`);
  } finally {
    if (unlisten) {
      unlisten();
    }
    setLoading(false);
  }
}
