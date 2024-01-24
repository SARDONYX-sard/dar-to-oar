import { render, fireEvent, screen, waitFor } from '@testing-library/react';

import { SelectPathButton, LogFileButton } from '.';

jest.mock('react-hot-toast', () => ({
  toast: {
    error: jest.fn(),
  },
}));
jest.mock('@/tauri_cmd', () => ({
  openPath: jest.fn(),
}));
jest.mock('@/hooks', () => ({
  useTranslation: () => ({ t: jest.fn() }),
}));

describe('LogFileButton component', () => {
  it('should call openLogFile on click and show error toast on failure', async () => {
    const openLogFileMock = jest.fn().mockRejectedValue('Error message');
    const tMock = jest.fn().mockReturnValue('Open Log');

    require('@/tauri_cmd').openLogFile = openLogFileMock;
    require('@/hooks').useTranslation = () => ({ t: tMock });

    render(<LogFileButton />);

    // Click the button
    fireEvent.click(screen.getByText('Open Log'));

    // Check if openLogFile is called
    expect(openLogFileMock).toHaveBeenCalled();

    // Wait for the asynchronous operation to complete
    await waitFor(() => {
      // Check if error toast is shown on failure
      expect(require('react-hot-toast').toast.error).toHaveBeenCalledWith('Error message');
    });
  });
});

describe('SelectPathButton component', () => {
  it('should call openPath on click and show error toast on failure', async () => {
    const openPathMock = jest.fn().mockRejectedValue('Error message');
    const tMock = jest.fn().mockReturnValue('Select');

    require('@/tauri_cmd').openPath = openPathMock;
    require('@/hooks').useTranslation = () => ({ t: tMock });

    render(<SelectPathButton path="/test" isDir setPath={jest.fn()} />);

    // Click the button
    fireEvent.click(screen.getByText('Select'));

    // Check if openPath is called
    expect(openPathMock).toHaveBeenCalled();

    // Wait for the asynchronous operation to complete
    await waitFor(() => {
      // Check if error toast is shown on failure
      expect(require('react-hot-toast').toast.error).toHaveBeenCalledWith('Error message');
    });
  });
});
