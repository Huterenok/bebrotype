import { useUnit } from "effector-react";
import { useRouter } from "next/navigation";

import { $user } from "enities/User/model";

export const useAuth = (): boolean => {
  const router = useRouter();
  const user = useUnit($user);

  if (!user) {
    router.replace("/");
    return false;
  }

  return true;
};
