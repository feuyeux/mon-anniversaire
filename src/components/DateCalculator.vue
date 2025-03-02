<template>
  <div class="date-calculator">
    <div class="calculation-type-selector">
      <label v-for="type in calculationTypes" :key="type.id" class="radio-label">
        <input type="radio" :value="type.id" v-model="selectedType" name="calculationType">
        <span>{{ type.label }}</span>
      </label>
    </div>

    <div v-if="selectedType === 'dateDiff'" class="calculation-panel">
      <h3>计算两个日期之间的天数</h3>
      <div class="form-group">
        <label for="start-date">开始日期:</label>
        <input id="start-date" type="date" v-model="startDate" class="form-input date-input">
      </div>

      <div class="form-group">
        <label for="end-date">结束日期:</label>
        <input id="end-date" type="date" v-model="endDate" class="form-input date-input">
      </div>

      <div class="form-actions">
        <button @click="calculateDateDifference" class="primary-button">计算</button>
      </div>

      <div v-if="dateDiffResult" class="result">
        {{ dateDiffResult }}
      </div>
    </div>

    <div v-if="selectedType === 'dateAdd'" class="calculation-panel">
      <h3>日期加减计算</h3>
      <div class="form-group">
        <label for="base-date">基准日期:</label>
        <input id="base-date" type="date" v-model="baseDate" class="form-input date-input">
      </div>

      <div class="form-group operation-group">
        <label>操作类型:</label>
        <div class="radio-group">
          <label class="radio-option">
            <input type="radio" value="add" v-model="addOrSubtract" name="operation">
            <span>增加</span>
          </label>
          <label class="radio-option">
            <input type="radio" value="subtract" v-model="addOrSubtract" name="operation">
            <span>减少</span>
          </label>
        </div>
      </div>

      <div class="form-group">
        <label for="days-count">天数:</label>
        <input id="days-count" type="number" v-model.number="daysToAddOrSubtract" min="0" class="form-input">
      </div>

      <div class="form-actions">
        <button @click="calculateDateAddSubtract" class="primary-button">计算</button>
      </div>

      <div v-if="dateAddResult" class="result">
        {{ dateAddResult }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import '../styles.css';
import { useDateCalculator } from './DateCalculator';

const {
  calculationTypes,
  selectedType,
  startDate,
  endDate,
  dateDiffResult,
  baseDate,
  daysToAddOrSubtract,
  addOrSubtract,
  dateAddResult,
  calculateDateDifference,
  calculateDateAddSubtract
} = useDateCalculator();
</script>