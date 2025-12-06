<script setup lang="ts">
  import { onMounted, ref } from 'vue'
  import { ToastAction, ToastDescription, ToastProvider, ToastRoot, ToastTitle, ToastViewport } from 'reka-ui'
  import { useSettingsStore } from '../stores/settings'

  const props = withDefaults(
    defineProps<{
      title?: string
      message?: string
      dismissible?: boolean
      autoDismiss?: boolean
      dismissLabel?: string
    }>(),
    {
      title: 'Title Here',
      message: '',
      autoDismiss: true,
      dismissible: false,
      dismissLabel: 'Dismiss',
    }
  )
  const settingsStore = useSettingsStore()
  const open = ref(true)

  onMounted(() => {
    if (props.autoDismiss) {
      setTimeout(() => {
        open.value = false
      }, 5000)
    }
  })
</script>

<template>
  <ToastProvider>
    <ToastRoot
      :key="new Date().getTime()"
      v-model:open="open"
      class="rounded-lg shadow-sm border p-4 grid [grid-template-areas:_'title_action'_'description_action'] grid-cols-[1fr_max-content] gap-x-4 gap-y-2 items-start data-[state=open]:animate-slideIn data-[state=closed]:animate-hide data-[swipe=move]:translate-x-[var(--reka-toast-swipe-move-x)] data-[swipe=cancel]:translate-x-0 data-[swipe=cancel]:transition-[transform_200ms_ease-out] data-[swipe=end]:animate-swipeOut"
      :style="{
        backgroundColor: settingsStore.colors.backgroundLight,
        borderColor: settingsStore.colors.border,
        color: settingsStore.colors.foreground,
      }"
    >
      <ToastTitle class="[grid-area:_title] font-medium text-base">
        {{ title }}
      </ToastTitle>
      <ToastDescription
        v-if="message"
        class="[grid-area:_description] text-sm break-words overflow-wrap-anywhere whitespace-normal max-h-[300px] overflow-y-auto pr-2"
      >
        {{ message }}
      </ToastDescription>
      <ToastAction
        v-if="dismissible"
        class="[grid-area:_action]"
        as-child
        alt-text="Vai alla pianificazione per annullare"
      >
        <button class="text-sm text-primary-500 hover:text-primary-400">
          {{ dismissLabel }}
        </button>
      </ToastAction>
    </ToastRoot>
    <ToastViewport
      class="fixed bottom-0 right-0 flex flex-col p-6 gap-3 w-auto max-w-[400px] m-0 list-none z-[2147483647] outline-none"
    />
  </ToastProvider>
</template>
