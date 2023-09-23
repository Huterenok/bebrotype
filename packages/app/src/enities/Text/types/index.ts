import { z } from "zod";

export type IText = z.infer<typeof TextContract>;

export const TextContract = z.object({
  id: z.number(),
  title: z.string(),
  content: z.string(),
  user_id: z.number(),
  likes: z.number(),
});

export const TextArrayContract = TextContract.array();

export interface ICreateTextRequest {
  title: string;
  content: string;
}

export interface IUpdateTextRequest {
  title: string;
  content: string;
}
