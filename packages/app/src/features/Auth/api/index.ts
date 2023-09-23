import { createEndpointUrl } from "shared/api";

export const register = createEndpointUrl("auth/register");

export const login = createEndpointUrl("auth/login");

//TODO: todoooooo
export const googleOAuth = () => {
  location.replace(
    //TODO: return_url
    createEndpointUrl("auth/google-oauth?return_url=http://localhost:3000")
  );
};
