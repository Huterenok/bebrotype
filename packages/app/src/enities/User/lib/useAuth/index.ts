"use client";

import { useUnit } from "effector-react";
import { useRouter } from "next/navigation";

import { $user } from "enities/User/model";
import { useEffect } from "react";

export const useAuth = () => {
  const router = useRouter();
  const user = useUnit($user);

  useEffect(() => {
    if (!user) {
      router.replace("/");
    }
  });
};
