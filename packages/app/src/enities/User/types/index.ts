export interface IUser {
		id: number;
    username: string;
    email: string;
    avatar?: string;
    near_address?: string;
    favourite_texts?: number[];
}