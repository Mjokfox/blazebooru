<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { useRoute } from "vue-router";

import MainLayout from "@/components/MainLayout.vue";
import { useWikiStore } from "@/stores/wiki";

import axios from "axios";
import { WikiPage } from "@/models/api/wiki";
import NewWikiForm from "@/components/wiki/NewWikiForm.vue";
import { useAuthStore } from "@/stores/auth";
import router from "@/router";

const route = useRoute();

const authStore = useAuthStore();

const wikiStore = useWikiStore();

const wikipages = ref<WikiPage[]>();
const error = ref<String|null>(null);

watch(route, () => {
  fetchWikiPages();
});

onMounted(async () => {
    await fetchWikiPages();
});

const toggleCreate = () => {
  if (!wikiStore.newWikiPage) {
    wikiStore.newWikiPage = {
        title: '',
        body: '',
        locked: false,
        reason: '',
        user_id: undefined
      };
  }
};

const fetchWikiPages = async () => {
  error.value = null;
  try {
    wikipages.value = await wikiStore.getWikiPageList();
  } catch (e) {
    if (axios.isAxiosError(e)) {
      if (e.response && e.response.status === 404) {
        error.value = 'No pages found';
      } else {
        error.value = 'An unexpected error occurred';
      }
    }
  }
};

const submit_new = async () => {
  const newpage = await wikiStore.createWiki();
  if (newpage) {
    router.push({ name: "wiki", params: { name: newpage?.title } });
  }
};

</script>

<template>
  <main>
    <MainLayout>
      <div v-if="wikipages" class="content">
        <div class="title">All wikis</div>
          <router-link v-for="w of wikipages"
              :key="w.id"
              :to="{ name: 'wiki', params: { name: w.title } }"
              class="wiki-link"
            >{{ w.title }}
          </router-link>
      </div>
      <div v-if="error">
        {{ error }}
      </div>
    <hr/>
    <button v-if="authStore.isAuthorized && !wikiStore.newWikiPage" class="edit-button link-button" @click="toggleCreate">
      <i class="fa-solid fa-pen-to-square"></i> create
    </button>
    <NewWikiForm v-if="wikiStore.newWikiPage" @submit="submit_new"></NewWikiForm>
    </MainLayout>
  </main>
</template>

<style scoped lang="scss">
main {
  padding: 1rem;
}

.wiki-link {
  margin: 1em;
}

.title {
  font-size: 2rem;
}
</style>
