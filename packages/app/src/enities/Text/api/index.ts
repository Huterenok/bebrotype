import { ICreateTextRequest, IUpdateTextRequest, IText} from "../types";

import { request } from "shared/api";

export const getTextsByUserId = async (id: number): Promise<IText[]> => {
  return await request<IText[]>({
    endpoint: `text/user/${id}`,
  });
};

export const getTextById = async (id: number): Promise<IText> => {
  return await request<IText>({
    endpoint: `text/${id}`,
  });
};

export const createText = async (body: ICreateTextRequest): Promise<IText> => {
  return await request<IText>({
    endpoint: "text",
    method: "POST",
    body,
  });
};

export const updateText = async (
  id: number,
  body: IUpdateTextRequest
): Promise<IText> => {
  return await request<IText>({
    endpoint: `text/edit/${id}`,
    method: "PATCH",
    body,
  });
};

export const getFavourite = async (): Promise<IText[]> => {
  return await request<IText[]>({
    endpoint: `text/favourite`,
    method: "GET",
  });
};

export const toggleFavourite = async (id: number): Promise<boolean> => {
  return await request<boolean>({
    endpoint: `text/favourite/${id}`,
    method: "PATCH",
  });
};

export const deleteFavourite = async (id: number): Promise<IText> => {
  return await request<IText>({
    endpoint: `text/delete/${id}`,
    method: "PATCH",
  });
};

