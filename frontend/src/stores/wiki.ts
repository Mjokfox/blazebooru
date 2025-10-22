import axios from "axios";
import { defineStore } from "pinia";
import { ref } from "vue";

import { useAuthStore } from "./auth";

import { WikiPage, UpdateWikiPage, NewWikiPage } from "@/models/api/wiki";

export const useWikiStore = defineStore("wiki", () => {
  const authStore = useAuthStore();

  const newWikiPage = ref<NewWikiPage>();
  const updateWikiPage = ref<UpdateWikiPage>();

  async function getWikiPage(wiki_name: string) {
    const res = await axios.get<WikiPage>(`/api/wiki/n/${wiki_name}`, {
      headers: await authStore.getAuthHeaders(),
    });

    return res.data;
  }

  async function getWikiPageList() {
    const res = await axios.get<WikiPage[]>('/api/wiki/list', {
      headers: await authStore.getAuthHeaders(),
    });

    return res.data;
  }

  async function createWiki() {

    if (newWikiPage.value === undefined
        || !authStore.isAuthorized 
        || authStore.userProfile === undefined
    ) {
        return;
    }

    newWikiPage.value.user_id = authStore.userProfile.id;

    const newWiki = await _createWiki(newWikiPage.value);

    newWikiPage.value = undefined;

    return newWiki;
  }

  async function _createWiki(wiki: NewWikiPage) {
    const res = await axios.post<WikiPage>(`/api/wiki/new`, wiki, {
      headers: await authStore.getAuthHeaders(),
    });

    return res.data;
  }

  async function updateWiki() {

    if (updateWikiPage.value === undefined) {
        return;
    }
  
    const res = await axios.post(`/api/wiki/n/${updateWikiPage.value.id}`, updateWikiPage.value, {
      headers: await authStore.getAuthHeaders(),
    });

    return res.status == 200;
  }

  async function deleteWiki(wiki_id: string) {
    await axios.delete(`/api/wiki/n/${wiki_id}`, {
      headers: await authStore.getAuthHeaders(),
    });

  }

  return {
    newWikiPage,
    updateWikiPage,
    getWikiPage,
    getWikiPageList,
    createWiki,
    updateWiki,
    deleteWiki
  };
});
