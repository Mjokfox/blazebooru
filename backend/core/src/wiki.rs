use std::net::IpAddr;

use blazebooru_models::view as vm;

use super::BlazeBooruCore;

impl BlazeBooruCore {
    pub async fn create_wiki_page(&self, user_id: i32, user_ip: IpAddr, new_wiki_page: vm::NewWikiPage) -> Result<vm::WikiPage, anyhow::Error> {
        let wiki_page = self.store.create_wiki_page(user_id, user_ip, &new_wiki_page.into()).await?;

        Ok(wiki_page.into())
    }

    pub async fn get_wiki_page(&self, wiki_id: i32) -> Result<Option<vm::WikiPage>, anyhow::Error> {
        let wiki_page = self.store.get_wiki_page(wiki_id).await?;

        Ok(wiki_page.map(vm::WikiPage::from))
    }

    pub async fn get_wiki_page_by_name(&self, name: String) -> Result<Option<vm::WikiPage>, anyhow::Error> {
        let wiki_page = self.store.get_wiki_page_by_name(name).await?;

        Ok(wiki_page.map(vm::WikiPage::from))
    }

    pub async fn get_all_wiki_pages(&self) -> Result<Vec<vm::WikiPage>, anyhow::Error> {
        let wiki_pages = self.store.get_all_wiki_pages().await?;

        Ok(wiki_pages.into_iter().map(vm::WikiPage::from).collect())
    }

    pub async fn update_wiki_page(&self, user_id: i32, user_ip: IpAddr, new_wiki_page: vm::UpdateWikiPage) -> Result<bool, anyhow::Error> {
        let res = self.store.update_wiki_page(user_id, user_ip, &new_wiki_page.into()).await?;

        Ok(res)
    }

    pub async fn delete_wiki_page(&self, name: String) -> Result<bool, anyhow::Error> {
        let success = self.store.delete_wiki_page(name).await?;

        Ok(success)
    }
}
