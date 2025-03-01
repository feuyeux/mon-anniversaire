import { invoke } from '@tauri-apps/api/core';
import type { Anniversary } from '../types';

export async function getAnniversaries(): Promise<Anniversary[]> {
  return await invoke('get_anniversaries');
}

export async function loadAnniversaries(): Promise<Anniversary[]> {
  try {
    return await getAnniversaries();
  } catch (error) {
    console.error('Error loading anniversaries:', error);
    return [];
  }
}

export async function createAnniversary(name: string, date: string): Promise<boolean> {
  try {
    await invoke('save_anniversary', { name, date });
    return true;
  } catch (error) {
    console.error('Error creating anniversary:', error);
    return false;
  }
}

export async function deleteAnniversary(id: number): Promise<boolean> {
  console.log('apiClient: Attempting to delete anniversary with ID:', id);
  try {
    // Make sure we're explicitly passing the id as a parameter
    await invoke('delete_anniversary', { id });
    console.log('apiClient: Delete operation completed successfully');
    return true;
  } catch (error) {
    console.error('apiClient: Error deleting anniversary:', error);
    return false;
  }
}

export async function updateAnniversary(id: number, name: string, date: string): Promise<boolean> {
  try {
    await invoke('update_anniversary', { id, name, date });
    return true;
  } catch (error) {
    console.error('Error updating anniversary:', error);
    return false;
  }
}