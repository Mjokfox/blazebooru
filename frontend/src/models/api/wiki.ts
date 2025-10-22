export interface WikiPage {
  id: number;
  creator_id: number;
  creator_name: string;
  title: string;
  body: string;
  locked: boolean;
  created_at: string;
  revision: number;
  updated_at: string;
  updater_id: number;
  updater_name: string;
}

export interface NewWikiPage {
  user_id: number | undefined;
  title: string;
  body: string;
  locked: boolean;
  reason: string;
}

export interface UpdateWikiPage extends NewWikiPage {
  id: number;
  deleted: boolean;
}