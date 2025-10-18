export interface User {
  id: number;
  created_at: string;
  name: string;
  rank: number;
  biography: string;
  css: string;
}

export interface UserUpdateUser {
  name: string;
  biography: string;
  css: string;
}

export interface AdminUpdateUser extends UserUpdateUser {
  rank: number;
}

