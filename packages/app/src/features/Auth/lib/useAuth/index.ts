"use client";

import { useEffect } from "react";
import { useUnit } from "effector-react";
import { useRouter } from "next/navigation";

import { $user } from "enities/User/model";

export const useAuth = () => {
  const router = useRouter();
  const user = useUnit($user);

  useEffect(() => {
    if (!user) {
      router.replace("/");
    }
  }, [router, user]);

  return user;
};
