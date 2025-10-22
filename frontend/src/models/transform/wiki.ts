import { UpdateWikiPage, WikiPage } from "../api/wiki";

export function update_wiki_into_current(up: UpdateWikiPage, updater_name: string, cur: WikiPage): WikiPage {
    return <WikiPage>{
        ...cur,
        title: up.title,
        body: up.body,
        updater_id: up.user_id,
        updater_name: updater_name,
        updated_at: new Date().toISOString(),
        revision: cur.revision + 1,
        locked: up.locked,
        reason: up.reason
    }
}