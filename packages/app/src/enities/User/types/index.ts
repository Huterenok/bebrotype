export interface IUser {
  id: number;
  username: string;
  avatar?: string;
  near_address?: string;
}

export interface IUpdateUser {
  email: string;
  username: string;
}
