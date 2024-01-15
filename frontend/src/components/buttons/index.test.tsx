import { render, fireEvent, screen, waitFor } from '@testing-library/react';

import { SelectPathButton, LogFileButton, RemoveOarBtn } from '.';

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

    render(<SelectPathButton path="/test" setValue={jest.fn()} />);

    // Click the button
    fireEvent.click(screen.getByText('Select'));

    // Check if openPath is called
    expect(openPathMock).toHaveBeenCalledWith('/test', expect.any(Function), false);

    // Wait for the asynchronous operation to complete
    await waitFor(() => {
      // Check if error toast is shown on failure
      expect(require('react-hot-toast').toast.error).toHaveBeenCalledWith('Error message');
    });
  });
});

describe('RemoveOarBtn component', () => {
  it('should call removeOarDir on click and show success toast on success', async () => {
    const removeOarDirMock = jest.fn();
    const tMock = jest.fn().mockReturnValue('Remove OAR');

    require('@/tauri_cmd').removeOarDir = removeOarDirMock;
    require('@/hooks').useTranslation = () => ({ t: tMock });

    render(<RemoveOarBtn darPath="/test-dar" oarPath="/test-oar" />);

    // Click the button
    fireEvent.click(screen.getByText('Remove OAR'));

    // Check if removeOarDir is called with the correct arguments
    expect(removeOarDirMock).toHaveBeenCalledWith('/test-oar');
  });

  it('should call removeOarDir on click with darPath if oarPath is empty', async () => {
    const removeOarDirMock = jest.fn();
    const tMock = jest.fn().mockReturnValue('Remove OAR');

    require('@/tauri_cmd').removeOarDir = removeOarDirMock;
    require('@/hooks').useTranslation = () => ({ t: tMock });

    render(<RemoveOarBtn darPath="/test-dar" oarPath="" />);

    // Click the button
    fireEvent.click(screen.getByText('Remove OAR'));

    // Check if removeOarDir is called with the correct arguments
    expect(removeOarDirMock).toHaveBeenCalledWith('/test-dar');
  });

  it('should show error toast on failure', async () => {
    const removeOarDirMock = jest.fn().mockRejectedValue('Error message');
    const tMock = jest.fn().mockReturnValue('Remove OAR');

    require('@/tauri_cmd').removeOarDir = removeOarDirMock;
    require('@/hooks').useTranslation = () => ({ t: tMock });

    render(<RemoveOarBtn darPath="/test-dar" oarPath="/test-oar" />);

    // Click the button
    fireEvent.click(screen.getByText('Remove OAR'));

    // Wait for the asynchronous operation to complete
    await waitFor(() => {
      // Check if error toast is shown on failure
      expect(require('react-hot-toast').toast.error).toHaveBeenCalledWith('Error message');
    });
  });
});
