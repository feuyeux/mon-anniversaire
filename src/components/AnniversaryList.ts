import { ref } from 'vue';
import { loadAnniversaries, deleteAnniversary } from '../utils/apiClient';
import type { Anniversary } from '../types';

export function useAnniversaryList(onEditAnniversary: (anniversary: Anniversary) => void) {
  const anniversaries = ref<Anniversary[]>([]);
  const errorMessage = ref('');
  const isDeleting = ref(false);

  async function loadAnniversariesList() {
    try {
      anniversaries.value = await loadAnniversaries();
    } catch (error) {
      errorMessage.value = '加载纪念日失败';
      console.error(error);
    }
  }

  async function handleDelete(id: number) {
    isDeleting.value = true;
    try {
      const success = await deleteAnniversary(id);
      if (success) {
        anniversaries.value = anniversaries.value.filter(a => a.id !== id);
      } else {
        errorMessage.value = '删除纪念日失败';
      }
    } catch (error) {
      errorMessage.value = '删除纪念日失败';
      console.error(error);
    } finally {
      isDeleting.value = false;
    }
  }

  function editAnniversary(anniversary: Anniversary) {
    onEditAnniversary(anniversary);
  }

  function getDayClass(days: number) {
    return days < 0 ? 'past-date' : 'future-date';
  }

  return {
    anniversaries,
    errorMessage,
    isDeleting,
    loadAnniversariesList,
    handleDelete,
    editAnniversary,
    getDayClass
  };
}