<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface Anniversary {
  name: string;
  date: string;
  days: number;
}

const anniversaries = ref<Anniversary[]>([]);
const newAnniversaryName = ref("");
const newAnniversaryDate = ref("");

const daysToCalculate = ref(0);
const isFuture = ref(true);
const calculatedDate = ref("");

async function loadAnniversaries() {
  try {
    anniversaries.value = await invoke("get_anniversaries");
  } catch (error) {
    console.error("Failed to load anniversaries:", error);
  }
}

async function saveAnniversary() {
  if (!newAnniversaryName.value || !newAnniversaryDate.value) return;

  try {
    await invoke("save_anniversary", {
      name: newAnniversaryName.value,
      date: newAnniversaryDate.value,
    });

    newAnniversaryName.value = "";
    newAnniversaryDate.value = "";

    // Reload the list
    await loadAnniversaries();
  } catch (error) {
    console.error("Failed to save anniversary:", error);
  }
}

async function calculateDate() {
  try {
    calculatedDate.value = await invoke("calculate_date", {
      days: Math.abs(daysToCalculate.value),
      isFuture: isFuture.value,
    });
  } catch (error) {
    console.error("Failed to calculate date:", error);
  }
}

onMounted(() => {
  loadAnniversaries();
});

function getDayClass(days: number): string {
  return days < 0 ? 'past-date' : 'future-date';
}
</script>

<template>
  <main class="container">
    <h1>Mon Anniversaire</h1>

    <div class="card">
      <h2>创建纪念日</h2>
      <form class="form" @submit.prevent="saveAnniversary">
        <div class="form-group">
          <label for="name">纪念日名称:</label>
          <input id="name" v-model="newAnniversaryName" placeholder="例如: 生日、结婚纪念日..." />
        </div>
        <div class="form-group">
          <label for="date">日期:</label>
          <input id="date" v-model="newAnniversaryDate" type="date" />
        </div>
        <button type="submit">保存</button>
      </form>
    </div>

    <div class="card">
      <h2>我的纪念日</h2>
      <div v-if="anniversaries.length === 0" class="no-data">
        还没有纪念日，请添加一个！
      </div>
      <ul v-else class="anniversary-list">
        <li v-for="(anniversary, index) in anniversaries" :key="index" class="anniversary-item">
          <div class="anniversary-info">
            <h3>{{ anniversary.name }}</h3>
            <p>日期: {{ anniversary.date }}</p>
            <p :class="getDayClass(anniversary.days)">
              {{ anniversary.days < 0 ? '已过去' : '还有' }} {{ Math.abs(anniversary.days) }} 天 </p>
          </div>
        </li>
      </ul>
    </div>

    <div class="card">
      <h2>计算日期</h2>
      <form class="form" @submit.prevent="calculateDate">
        <div class="form-group">
          <label for="days">天数:</label>
          <input id="days" v-model.number="daysToCalculate" type="number" min="0" />
        </div>
        <div class="form-group">
          <label>方向:</label>
          <div class="radio-group">
            <label>
              <input type="radio" v-model="isFuture" :value="true" />
              未来
            </label>
            <label>
              <input type="radio" v-model="isFuture" :value="false" />
              过去
            </label>
          </div>
        </div>
        <button type="submit">计算</button>
      </form>

      <div v-if="calculatedDate" class="result">
        <h3>计算结果:</h3>
        <p>{{ isFuture ? '未来' : '过去' }} {{ daysToCalculate }} 天是: {{ calculatedDate }}</p>
      </div>
    </div>
  </main>
</template>

<style scoped>
.container {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem;
  font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
}

h1 {
  text-align: center;
  margin-bottom: 2rem;
}

.card {
  background: #f5f5f5;
  border-radius: 8px;
  padding: 1.5rem;
  margin-bottom: 2rem;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

h2 {
  margin-top: 0;
  margin-bottom: 1.5rem;
  color: #333;
}

.form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

label {
  font-weight: 500;
}

input {
  padding: 0.5rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 1rem;
}

button {
  padding: 0.5rem 1rem;
  background: #3399ff;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 1rem;
  cursor: pointer;
  margin-top: 0.5rem;
}

button:hover {
  background: #2288ee;
}

.anniversary-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.anniversary-item {
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 1rem;
  background: white;
}

.no-data {
  text-align: center;
  color: #666;
  font-style: italic;
}

.past-date {
  color: #ff9900;
  font-weight: 500;
}

.future-date {
  color: #33cc66;
  font-weight: 500;
}

.radio-group {
  display: flex;
  gap: 1rem;
}

.result {
  margin-top: 1.5rem;
  padding-top: 1rem;
  border-top: 1px solid #ddd;
}

.result h3 {
  margin-top: 0;
}
</style>