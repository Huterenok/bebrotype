"use client";

import { useSearchParams, useRouter } from "next/navigation";
import { ReactNode, useLayoutEffect } from "react";
import { useUnit } from "effector-react";

import { whoamiEv, setUserToken } from "enities/Session";

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const whoami = useUnit(whoamiEv);
  const params = useSearchParams();
  const router = useRouter();

  useLayoutEffect(() => {
    const userToken = params.get("token");

    if (userToken) {
      setUserToken(userToken);
      router.replace("/");
    }

    whoami();
  }, [params, router]);

  return <>{children}</>;
};
