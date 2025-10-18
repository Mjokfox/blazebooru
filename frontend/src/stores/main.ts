import { useStorage } from "@vueuse/core";
import axios from "axios";
import { defineStore } from "pinia";
import { computed, ref } from "vue";

import { useAuthStore } from "./auth";

import type { PageInfo, Post, UpdatePost } from "@/models/api/post";
import type { SysConfig } from "@/models/api/sys";
import type { AdminUpdateUser, User, UserUpdateUser } from "@/models/api/user";
import { DEFAULT_SETTINGS, type Settings } from "@/models/settings";

export interface Search {
  tags: string[];
  exclude_tags: string[];
}

const CALCULATE_PAGES = 12;

const EMPTY_SEARCH: Search = {
  tags: [],
  exclude_tags: [],
};

export const useMainStore = defineStore("main", () => {
  const authStore = useAuthStore();

  const sysConfig = ref<SysConfig>();

  const activeSearch = ref<Search>(EMPTY_SEARCH);

  let calculatedPages: Record<number, PageInfo> = {};
  const lastPage = ref<PageInfo>();
  const currentPage = ref(-1);
  const currentPosts = ref<Post[]>([]);
  const settings = useStorage<Settings>("bb_settings", DEFAULT_SETTINGS, undefined, { mergeDefaults: true });

  const pageCount = computed(() => lastPage.value?.no || 0);

  const currentTags = computed(() => {
    const tags: string[] = [];

    // Collect all distinct tags from every currently shown post
    for (const p of currentPosts.value) {
      tags.push(...p.tags.filter((tag) => tags.findIndex((t) => t === tag) < 0));
    }

    // Sort tags alphabetically
    tags.sort((a, b) => a.localeCompare(b));

    return tags;
  });

  async function fetchSysConfig() {
    const res = await axios.get<SysConfig>("/api/sys/config");

    sysConfig.value = res.data;
  }

  function getPageNumbers(max_pages: number) {
    const half_max_pages = max_pages / 2;

    return computed(() => {
      const pages: number[] = [];

      let first_page = Math.max(1, currentPage.value - half_max_pages);
      let last_page = Math.min(pageCount.value, currentPage.value + half_max_pages);

      const page_diff = last_page - first_page;
      if (page_diff < max_pages) {
        if (first_page === 1) {
          last_page = Math.min(pageCount.value, last_page + (max_pages - page_diff));
        } else {
          first_page = Math.max(1, first_page - (max_pages - page_diff));
        }
      }

      for (let i = first_page; i <= last_page; ++i) {
        pages.push(i);
      }

      return pages;
    });
  }

  function clearSearch() {
    activeSearch.value = EMPTY_SEARCH;
  }

  async function getPost(id: number) {
    const res = await axios.get<Post>(`/api/post/${id}`, {
      headers: await authStore.getAuthHeaders(),
    });

    const post = res.data;

    const existingIndex = currentPosts.value.findIndex((p) => p.id === id);
    if (existingIndex >= 0) {
      currentPosts.value[existingIndex] = post;
    }

    return res.data;
  }

  async function fetchPosts(include_tags: string[], exclude_tags: string[], start_id: number) {
    const t = include_tags.join(",") || undefined;
    const e = exclude_tags.join(",") || undefined;

    const res = await axios.get<Post[]>("/api/post", {
      params: {
        t,
        e,
        sid: start_id,
        limit: settings.value.posts_per_page,
      },
      headers: await authStore.getAuthHeaders(),
    });

    return res.data;
  }

  async function getPublicUserProfile(username: string) {
    const res = await axios.get<User>("/api/user/pubprofile", {
      params: {
        name: username,
      }
    });

    return res.data;
  }

  async function getAllPublicUsers() {
    const res = await axios.get<User[]>("/api/user/all");

    return res.data;
  }

  async function updateUser(id: number, update_post: UserUpdateUser | AdminUpdateUser) {
    await axios.post(`/api/user/${id}`, update_post, {
      headers: await authStore.getAuthHeaders(),
    });

    return true;
  }

  async function deleteUser(id: number) {
    await axios.delete(`/api/user/${id}`, {
      headers: await authStore.getAuthHeaders(),
    });

    return true;
  }

  async function calculatePages(origin_page?: PageInfo, page_count?: number) {
    const search = activeSearch.value;
    if (!search) {
      return;
    }

    const t = search.tags.join(",") || undefined;
    const e = search.exclude_tags.join(",") || undefined;

    const res = await axios.get<PageInfo[]>("/api/post/pages", {
      params: {
        t,
        e,
        ppp: settings.value.posts_per_page,
        pc: page_count || CALCULATE_PAGES,
        opno: origin_page?.no,
        opsid: origin_page?.start_id,
      },
    });

    addCalculatedPages(res.data);
  }

  async function calculateLastPage() {
    const search = activeSearch.value;
    if (!search) {
      return;
    }

    const t = search.tags.join(",") || undefined;
    const e = search.exclude_tags.join(",") || undefined;

    const res = await axios.get<PageInfo>("/api/post/pages/last", {
      params: {
        t,
        e,
        ppp: settings.value.posts_per_page,
      },
    });

    lastPage.value = res.data;
    addCalculatedPages([res.data]);
  }

  async function updatePost(id: number, update_post: UpdatePost) {
    await axios.post(`/api/post/${id}/update`, update_post, {
      headers: await authStore.getAuthHeaders(),
    });

    return true;
  }

  async function deletePost(id: number) {
    await axios.delete(`/api/post/${id}`, {
      headers: await authStore.getAuthHeaders(),
    });

    // Remove the post from current posts
    const index = currentPosts.value.findIndex((p) => p.id === id);
    if (index >= 0) {
      currentPosts.value.splice(index, 1);
    }

    return true;
  }

  async function loadPosts(start_id: number) {
    const search = activeSearch.value;
    if (!search) {
      return;
    }

    currentPosts.value = await fetchPosts(search.tags, search.exclude_tags, start_id);
  }

  function findNearestPage(page: number) {
    const nearestPages = (Object.values(calculatedPages) as unknown as PageInfo[]).sort(
      (a, b) => Math.abs(page - a.no) - Math.abs(page - b.no),
    );

    const nearestPage = nearestPages[0];
    if (!nearestPage) {
      return;
    }

    if (nearestPage.no < page) {
      const stopAtPage = nearestPages.find((pi) => pi.no > page);

      const toPageNo = stopAtPage && stopAtPage.no < page + CALCULATE_PAGES ? stopAtPage.no : page + CALCULATE_PAGES;
      return { nearestPage, length: toPageNo - nearestPage.no };
    } else {
      const startAtPage = nearestPages.find((pi) => pi.no < page);

      const fromPageNo =
        startAtPage && startAtPage.no > page - CALCULATE_PAGES ? startAtPage.no : page - CALCULATE_PAGES;
      return { nearestPage, length: fromPageNo - nearestPage.no };
    }
  }

  async function getPage(page: number) {
    const pageInfo = calculatedPages[page];
    if (pageInfo) {
      return pageInfo;
    }

    const nearestPage = findNearestPage(page);
    if (nearestPage) {
      const { nearestPage: originPage, length } = nearestPage;
      await calculatePages(originPage, length);
    } else {
      await calculatePages(undefined, page + CALCULATE_PAGES);
    }

    return calculatedPages[page];
  }

  async function loadPage(page: number, force = false) {
    // Make sure the page being loaded is a valid number
    page = Math.max(1, Math.min(page, pageCount.value));

    // We are already on this page, do nothing.
    if (!force && currentPage.value === page) {
      return;
    }

    const pageInfo = await getPage(page);
    if (!pageInfo) {
      return;
    }

    currentPage.value = page;
    await loadPosts(pageInfo.start_id);
  }

  async function loadLastPage() {
    await loadPage(pageCount.value);
  }

  function addCalculatedPages(pages: PageInfo[]) {
    for (const p of pages) {
      calculatedPages[p.no] = p;
    }
  }

  async function refresh() {
    lastPage.value = undefined;
    calculatedPages = [];
    currentPosts.value = [];

    await calculateLastPage();
    if (currentPage.value > 0) {
      await loadPage(currentPage.value, true);
    }
  }

  function setSearch(search: Search) {
    activeSearch.value = search;
    currentPage.value = -1;
    lastPage.value = undefined;
    calculatedPages = [];
    currentPosts.value = [];
  }

  async function searchPosts(search: Search) {
    setSearch(search);

    if (sysConfig.value?.require_login && !authStore.isAuthorized) {
      return;
    }

    await calculateLastPage();
    await loadPage(1);
  }

  async function initialize() {
    await fetchSysConfig();
    await setSearch(EMPTY_SEARCH);
  }

  const initializePromise = initialize();

  async function isInitialized() {
    await initializePromise;
  }

  return {
    sysConfig,
    activeSearch,
    currentPage,
    pageCount,
    currentPosts,
    currentTags,
    settings,
    getPageNumbers,
    clearSearch,
    getPost,
    getPublicUserProfile,
    getAllPublicUsers,
    updateUser,
    deleteUser,
    loadPage,
    loadLastPage,
    refresh,
    searchPosts,
    updatePost,
    deletePost,
    isInitialized,
  };
});
