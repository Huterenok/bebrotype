export interface IUser {
  id: number;
  username: string;
  email: string;
  avatar?: string;
  near_address?: string;
}

export interface IUpdateUser {
  email: string;
  username: string;
}
