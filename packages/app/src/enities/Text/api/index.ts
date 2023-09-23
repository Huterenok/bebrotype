import { createEndpointUrl } from "shared/api";

export const createText = createEndpointUrl("text");

export const getAllTexts = (id: number): string => {
  return createEndpointUrl(`text?limit=${id}`);
};

export const getTextsByUserId = (id: number): string => {
  return createEndpointUrl(`text/user/${id}`);
};

export const getTextById = (id: number): string => {
  return createEndpointUrl(`text/${id}`);
};

export const getFavourite = createEndpointUrl(`text/favourite`);

export const toggleFavourite = (id: number): string => {
  return createEndpointUrl(`text/favourite/${id}`);
};

export const deleteFavourite = (id: number): string => {
  return createEndpointUrl(`text/delete/${id}`);
};

export const updateText = (id: number): string => {
  return createEndpointUrl(`text/edit/${id}`);
};
