import { IUser } from "enities/User";
import { request, requestFD } from "shared/api";

export const getAllUsers = async (limit: number): Promise<IUser[]> => {
  let query = `?limit=${limit}`;
  return await request<IUser[]>({ endpoint: `user/all${query}` });
};

export const getUser = async (id: number): Promise<IUser> => {
  return await request<IUser>({ endpoint: `user/${id}` });
};

//TODO
export const updateUser = async (formData: FormData): Promise<IUser> => {
  return await requestFD<IUser>({
    endpoint: "user/edit",
    body: formData
  });
};
