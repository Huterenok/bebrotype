import { useSearchParams, useRouter } from "next/navigation";

import { setToken } from "shared/lib";

export const useOAuth = () => {
  const params = useSearchParams();
  const router = useRouter();

  const token = params.get("token");
  if (token) {
    setToken(token);
    router.replace("/");
  } else {
    return;
  }
};
