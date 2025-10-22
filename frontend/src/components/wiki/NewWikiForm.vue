<script setup lang="ts">

import { useWikiStore } from "@/stores/wiki";
import MarkdownEditor from "../common/MarkdownEditor.vue";

const emit = defineEmits<{
  (e: "submit"): void;
}>();

const wikiStore = useWikiStore();

const submit =  () => {
  emit("submit");
}

</script>

<template>
    <form v-if="wikiStore.newWikiPage" class="wiki-create-form" @submit.prevent="submit">
      <label>Name or tag</label>
      <input name="new_wiki_title" type="text" placeholder="Name..." required v-model="wikiStore.newWikiPage.title">
      <MarkdownEditor v-model="wikiStore.newWikiPage.body" :placeholder="'Content... \nUse markdown!'"/>
      <label>Why does this wiki need to exist?</label>
      <input name="new_wiki_title" type="text" placeholder="Reason..." v-model="wikiStore.newWikiPage.reason">
      <div>
        <input name="new_wiki_title" type="checkbox" v-model="wikiStore.newWikiPage.locked"><label> Make it locked?</label>
      </div>
      <div class="form-buttons">
        <input class="save-button" type="submit" value="Submit" />
      </div>
    </form>
</template>

<style scoped lang="scss">
.wiki-create-form {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
}

</style>
