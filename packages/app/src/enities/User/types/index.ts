import { z } from "zod";

export const UserContract = z.object({
  id: z.number(),
  username: z.string(),
  email: z.string(),
  avatar: z.string().nullish(),
  near_address: z.string().nullish(),
});

export type IUser = z.infer<typeof UserContract>;
