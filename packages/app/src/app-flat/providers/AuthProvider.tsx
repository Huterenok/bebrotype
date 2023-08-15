"use client";

import { useUnit } from "effector-react";
import { whoamiFn } from "enities/User";
import { ReactNode, useEffect } from "react";

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const whoami = useUnit(whoamiFn);

  useEffect(() => {
    whoami();
  }, []);

  return <>{children}</>;
};
