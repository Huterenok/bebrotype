import { request } from "shared/api";
import { IText } from "../types";

export const createText = async (id: number) => {
  return await request<IText[]>({ endpoint: `text`, method: "POST" });
};

export const getAllTexts = async (limit: number) => {
  return await request<IText[]>({ endpoint: `text?limit=${limit}` });
};

export const getTextsByUserId = async (id: number) => {
  return await request<IText[]>({ endpoint: `text/user/${id}` });
};

export const getTextById = async (id: number) => {
  return await request<IText>({ endpoint: `text/${id}` });
};

export const getRandomText = async () => {
  return await request<IText>({ endpoint: "text/random" });
};

export const getFavourite = async (token: string) => {
  return await request<IText[]>({ endpoint: `text/favourite`, token });
};

export const toggleFavourite = async (id: number, token: string) => {
  return await request({ endpoint: `text/favourite/${id}`, token });
};

export const deleteFavourite = async (id: number, token: string) => {
  return await request({
    endpoint: `text/delete/${id}`,
    token,
    method: "DELETE",
  });
};

export const updateText = async (id: number, token: string) => {
  return await request({ endpoint: `text/edit/${id}`, token, method: "PATCH" });
};
