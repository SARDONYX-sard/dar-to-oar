import { Toaster as Toast, type ToasterProps } from 'react-hot-toast';

/**
 * Toaster wrapper with default style applied
 * # INFO
 * This was written for ease of changing the source code.
 */
export function Toaster(props: Readonly<ToasterProps>) {
  return (
    <Toast
      position="bottom-right"
      reverseOrder={false}
      toastOptions={{
        style: {
          color: '#fff',
          background: '#1a1919e1',
        },
      }}
      {...props}
    />
  );
}
