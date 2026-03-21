<script setup lang="ts">
  import { ref, watch } from 'vue'
  import { useSettingsStore } from '../stores/settings'

  const props = defineProps({
    modelValue: [String, Number],
    placeholder: String,
    rows: {
      type: [String, Number],
      default: 4,
    },
  })

  const settingsStore = useSettingsStore()

  const inputValue = ref(props.modelValue)

  const emit = defineEmits(['update:modelValue'])

  const emitChange = () => {
    emit('update:modelValue', inputValue.value)
  }

  watch(
    () => props.modelValue,
    newValue => {
      inputValue.value = newValue
    }
  )
</script>

<template>
  <textarea
    v-model="inputValue"
    @input="emitChange"
    :rows="rows"
    :placeholder="placeholder"
    class="w-full text-sm outline py-2 px-2 focus:!outline-primary-500 rounded-md resize-y"
    :style="{
      backgroundColor: settingsStore.colors.backgroundLight,
      color: settingsStore.colors.foreground,
      outlineColor: settingsStore.colors.border,
    }"
    v-bind="$attrs"
  ></textarea>
</template>
