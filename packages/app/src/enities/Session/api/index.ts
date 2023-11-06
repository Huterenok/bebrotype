import { ISession } from "../types";

import { request } from "shared/api";

export const whoami = async (token: string): Promise<ISession> => {
  return await request<ISession>({ endpoint: "user/whoami", token });
};
