"use client";
import { ReactNode, useEffect } from "react";
import { useUnit } from "effector-react";

import { useOAuth } from "features/Auth";
import { whoamiFn } from "enities/User";

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const whoami = useUnit(whoamiFn);
  const _ = useOAuth();

  useEffect(() => {
    whoami();
  }, []);

  return <>{children}</>;
};
