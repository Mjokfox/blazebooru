<script setup lang="ts">
import { format, isSameDay, isSameYear, parseJSON } from "date-fns";
import VueMarkdown from 'vue-markdown-render'
import { onMounted, ref, toRefs, watch } from "vue";
import { useRoute, useRouter } from "vue-router";

import ConfirmDialog from "@/components/common/ConfirmDialog.vue";
import MainLayout from "@/components/MainLayout.vue";

import { useAuthStore } from "@/stores/auth";
import { useWikiStore } from "@/stores/wiki";

import { WikiPage } from "@/models/api/wiki";
import axios from "axios";
import UserLink from "@/components/user/UserLink.vue";
import UpdateWikiForm from "@/components/wiki/UpdateWikiForm .vue";
import { WikiError } from "@/models/api/error";
import NewWikiForm from "@/components/wiki/NewWikiForm.vue";
import { update_wiki_into_current } from "@/models/transform/wiki";


const props = defineProps<{
  name: string;
}>();

const { name } = toRefs(props);

const confirmDelete = ref<typeof ConfirmDialog>();

const route = useRoute();
const router = useRouter();

const authStore = useAuthStore();
const wikiStore = useWikiStore();

const wikiPage = ref<WikiPage>();
const error = ref<WikiError>( <WikiError> {type: 0} );

watch(route, () => {
  fetchWikiPage();
});

onMounted(async () => {
    await fetchWikiPage();
});

const fetchWikiPage = async () => {
  error.value = <WikiError> {type: 0}
  try {
    wikiPage.value = await wikiStore.getWikiPage(name.value);
    
  } catch (e) {
    if (axios.isAxiosError(e)) {
      if (e.response) {
        switch (e.response.status) {
          case 404:
            error.value.context = 'Wiki not found';
            error.value.type = 1;
            break;
          case 403:
            error.value.context = 'This wiki page is deleted';
            error.value.type = 2;
            router.push({ name: "wiki-list" });
            break;
          default:
            error.value.context = 'An unexpected error occurred';
            error.value.type = 100;
            break;
        }
      }
    }
  }
};

const toggleEdit = () => {
  if (!wikiStore.updateWikiPage && wikiPage.value) {
    wikiStore.updateWikiPage = {
        id: wikiPage.value.id,
        title: wikiPage.value.title,
        body: wikiPage.value.body,
        locked: wikiPage.value.locked,
        reason: '',
        user_id: undefined,
        deleted: false
      };
  } else if (!wikiStore.newWikiPage && error.value.type == 1) {
    wikiStore.newWikiPage = {
        title: name.value,
        body: '',
        locked: false,
        reason: '',
        user_id: undefined
      };
  }
};

const submit_update = async () => {
  if (!wikiStore.updateWikiPage || !wikiPage.value) {
    return;
  }
  wikiStore.updateWikiPage.id = wikiPage.value.id;
  const res = await wikiStore.updateWiki();
  if (res) {
    if (wikiStore.updateWikiPage.title != wikiPage.value.title) {
      router.push({ name: "wiki", params: { name: wikiStore.updateWikiPage.title } });
    } else {
      wikiPage.value = update_wiki_into_current(wikiStore.updateWikiPage, authStore.userProfile?.name ?? "", wikiPage.value);
    }
    wikiStore.updateWikiPage = undefined;
  }
};

const submit_new = async () => {
  const newpage = await wikiStore.createWiki();
  if (newpage) {
    router.push({ name: "wiki", params: { name: newpage?.title } });
  }
};

const deleteWiki = async () => {
  if (authStore.isAdmin && wikiPage.value){
    await wikiStore.deleteWiki(wikiPage.value.title);
    // Navigate back to the list view
    router.push({ name: "wiki-list" });
  }
};

const formatTime = (value: string): string => {
  const now = new Date();
  const time = parseJSON(value);

  if (isSameDay(now, time)) {
    // If time is today, omit the date
    return format(time, "HH:mm:ss");
  } else if (isSameYear(now, time)) {
    // If time is not today, but this year, include date without year
    return format(time, "MMM do, HH:mm:ss");
  }

  // If time is not this year, include full date with year
  return format(time, "MMM do yyyy, HH:mm:ss");
};

const EMPTY_PAGE = "This wiki page has not been created yet";

</script>

<template>
  <main>
    <MainLayout>
        <div v-if="wikiPage || error?.type == 1" class="content">
          <div class="title">{{ wikiPage?.title ?? name }}</div>
          <vue-markdown :source="wikiPage?.body ?? EMPTY_PAGE" />
          <hr/>
          <div v-if="wikiPage" class="wiki-footer">
            <a class="revisions-link">Revision {{ wikiPage.revision }}</a>
            by 
            <UserLink :name="wikiPage.updater_name">{{ wikiPage.updater_name }}</UserLink>
            at 
            <span class="time" title="Timestamp">{{ formatTime(wikiPage.updated_at) }}</span>
            <span v-if="wikiPage.updater_id !== wikiPage.creator_id"> 
              (originally created by 
                <UserLink :name="wikiPage.creator_name">{{ wikiPage.creator_name }}</UserLink>  
              ) 
            </span>
            <hr/>
          </div>
          <button v-if="authStore.isAuthorized && !wikiStore.updateWikiPage" class="edit-button link-button" @click="toggleEdit">
            <i class="fa-solid fa-pen-to-square"></i> Edit
          </button>
          <button v-if="authStore.isAdmin" class="delete-button link-button" @click="confirmDelete?.show()">
            <i class="fa-solid fa-trash"></i> Delete this wiki
          </button>
          <UpdateWikiForm v-if="wikiPage" :wiki_id="wikiPage.id" @submit="submit_update" />
          <NewWikiForm v-if="error.type == 1" @submit="submit_new"/>
          <ConfirmDialog v-if="authStore.isAdmin" ref="confirmDelete" title="Confirm delete" @confirm="deleteWiki">
            <template v-slot:confirm><i class="fa-solid fa-trash"></i> Delete</template>
            <div class="confirm-delete-dialog">Are you sure you want to delete the wiki "{{ wikiPage?.title }}"?</div>
          </ConfirmDialog>
        </div>
        <div v-if="error && error.type > 2">
            {{ error }}
        </div>
    </MainLayout>
  </main>
</template>

<style scoped lang="scss">
main {
  padding: 2rem;
}

.content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
}

.title {
  font-size: 2rem;
}
</style>
