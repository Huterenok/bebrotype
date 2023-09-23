import { z } from "zod";

export const ProfileContract = z.object({
  id: z.number(),
  username: z.string(),
  email: z.string(),
  avatar: z.string().nullish(),
  near_address: z.string().nullish(),
});

export type IProfile = z.infer<typeof ProfileContract>;

export interface IUpdateProfile {
  email: string;
  username: string;
}
