import { createEndpointUrl } from "shared/api";

export const getAllProfiles = (limit: number): string => {
  return createEndpointUrl(`user/all?limit=${limit}`);
};

export const getProfile = (id: number): string =>
  createEndpointUrl(`user/${id}`);

export const updateProfile = createEndpointUrl("user/edit");
