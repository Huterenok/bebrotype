export interface IText {
	id: number;
  title: string;
  content: string;
  user_id: number;
	likes: number;
}

export interface ICreateTextRequest {
	title: string;
	content: string;
}

export interface IUpdateTextRequest {
	title: string;
	content: string;
}