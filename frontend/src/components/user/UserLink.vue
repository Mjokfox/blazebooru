<script setup lang="ts">
import { toRefs } from "vue";

const props = defineProps<{
  name: string
}>();

const { name } = toRefs(props);

</script>

<template>
    <router-link :to="{ name: 'user', params: { name: name}}">
      <slot></slot>
    </router-link>
</template>
