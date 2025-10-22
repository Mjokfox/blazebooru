<script setup lang="ts">
import { ref, toRefs } from 'vue';
import VueMarkdown from 'vue-markdown-render';

const model = defineModel({ default: "", required: true });

const props = defineProps<{
  placeholder: string;
}>();

const { placeholder } = toRefs(props);

let previewing = ref<boolean>(false);

const preview = () => {
    previewing.value = true;
}

const edit = () => {
    previewing.value = false;
}

</script>

<template>
  <div class="markdown-editor">
    <div class="md-edit markdown-buttons-container">
        <button type="button" class="markdown-preview-button" @click="edit">Edit</button>
        <button type="button" class="markdown-preview-button" @click="preview">Preview</button>
    </div>
    <textarea v-if="!previewing"
        class="md-edit markdown-field"
        name="user_biography"
        v-model="model"
        :placeholder="placeholder"
        wrap="soft"></textarea>
    <div v-if="previewing" class="md-edit markdown-preview">
        <vue-markdown :source="model"/>
    </div>
  </div>
</template>

<style scoped lang="scss">
.markdown-editor {
    display: flex;
    flex-direction: column;
}

.md-edit {
    width: 50rem;
}

.markdown-preview-button {
    width: fit-content;
}

.markdown-field {
  resize: both;

  width: 50rem;
  height: 6rem;

  max-width: 100%;
}
.markdown-preview {
    min-height: 6rem;
    border: 1px solid #888;
}

</style>
