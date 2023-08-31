"use client";

import { useSearchParams, useRouter } from "next/navigation";
import { ReactNode, useLayoutEffect } from "react";
import { useUnit } from "effector-react";

import { whoamiFn } from "enities/User";

import { setToken } from "shared/config/token";

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const whoami = useUnit(whoamiFn);
  const params = useSearchParams();
  const router = useRouter();

  useLayoutEffect(() => {
    const token = params.get("token");

    if (token) {
      setToken(token);
      router.replace("/");
    }

    whoami();
  }, []);

  return <>{children}</>;
};
