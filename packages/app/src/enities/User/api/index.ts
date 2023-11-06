import { IUser } from "../types";

import { request } from "shared/api";

export const getAllUsers = async (limit: number) => {
  return await request<IUser>({ endpoint: `user/all?limit=${limit}` });
};

export const getUser = async (id: number) => {
  return await request<IUser>({ endpoint: `user/${id}` });
};
