"use client";

import { useEffect } from "react";
import { useUnit } from "effector-react";
import { useRouter } from "next/navigation";

import { $session } from "enities/Session";

export const useAuth = () => {
  const router = useRouter();
  const user = useUnit($session);

  useEffect(() => {
    if (!user) {
      router.replace("/");
    }
  }, [router, user]);

  return user!;
};
